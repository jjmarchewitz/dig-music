use crate::columns as col;
use crate::{error::LoadError, Play};
use ::zip;
use polars::prelude::*;
use rayon::prelude::*;
use regex::Regex;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

macro_rules! struct_to_dataframe {
    ($input:expr, [$($field:ident),+]) => {
        {
            // Extract the field values into separate vectors
            $(let mut $field = Vec::new();)*

            for e in $input.into_iter() {
                $($field.push(e.$field);)*
            }
            df! {
                $(stringify!($field) => $field,)*
            }
        }
    };
}

pub fn load_plays_to_df(path_to_zip: PathBuf) -> Result<DataFrame, LoadError> {
    let archive = load_archive(&path_to_zip)?;
    let plays = extract_song_data(archive)?;

    let mut df = struct_to_dataframe!(
        plays,
        [
            conn_country,
            episode_name,
            episode_show_name,
            incognito_mode,
            ip_addr_decrypted,
            master_metadata_album_album_name,
            master_metadata_album_artist_name,
            master_metadata_track_name,
            ms_played,
            offline,
            offline_timestamp,
            platform,
            reason_end,
            reason_start,
            shuffle,
            skipped,
            spotify_episode_uri,
            spotify_track_uri,
            ts,
            user_agent_decrypted,
            username
        ]
    )?;

    rename_columns(&mut df)?;

    df = df
        .lazy()
        .with_columns([col(col::TIMESTAMP).cast(DataType::Datetime(TimeUnit::Microseconds, None))])
        .collect()?;

    Ok(df)
}

fn load_archive<P: AsRef<Path>>(path: &P) -> Result<zip::ZipArchive<File>, LoadError> {
    let Ok(file) = File::open(path) else {
        return Err(LoadError::UnableToOpenFile);
    };

    let Ok(zip) = zip::ZipArchive::new(file) else {
        return Err(LoadError::UnableToLoadZipData);
    };

    Ok(zip)
}

// TODO: Have multiple versions of this for different versions of the .zip that Spotify spits out
fn extract_song_data(mut archive: zip::ZipArchive<File>) -> Result<Vec<Play>, LoadError> {
    let re = Regex::new(r"Streaming_History_Audio_.*\.json")?;

    let song_data_file_names: Vec<String> = archive
        .file_names()
        .par_bridge()
        .filter_map(|file_name| {
            if let Some(_) = re.find(file_name) {
                Some(file_name.to_string())
            } else {
                None
            }
        })
        .collect();

    // This cannot be made parallel because we can't have concurrent access to a single mutable reference.
    // In order to be parallel, the archive would need to have its contents read from multiple places at once.
    let file_contents: Vec<String> = song_data_file_names
        .into_iter()
        .map(|file_name| {
            let Ok(mut file) = archive.by_name(&file_name) else {
                return Err(LoadError::UnableToLoadZipData);
            };

            let mut content = String::default();

            if let Err(_) = file.read_to_string(&mut content) {
                return Err(LoadError::UnableToLoadZipData);
            }

            Ok(content)
        })
        .collect::<Result<Vec<String>, LoadError>>()?;

    let plays: Vec<Play> = file_contents
        .into_par_iter()
        .map(
            |contents| match serde_json::from_str::<Vec<Play>>(&contents) {
                Ok(v) => Ok(v),
                Err(_) => Err(LoadError::ParseError),
            },
        )
        .collect::<Result<Vec<Vec<Play>>, LoadError>>()?
        .into_par_iter()
        .flatten()
        .collect();

    Ok(plays)
}

fn rename_columns(df: &mut DataFrame) -> PolarsResult<&mut DataFrame> {
    Ok(df
        .rename("conn_country", col::CONN_COUNTRY)?
        .rename("episode_show_name", col::PODCAST_NAME)?
        .rename("ip_addr_decrypted", col::IP_ADDRESS)?
        .rename("master_metadata_album_album_name", col::ALBUM_NAME)?
        .rename("master_metadata_album_artist_name", col::ARTIST_NAME)?
        .rename("master_metadata_track_name", col::TRACK_NAME)?
        .rename("ts", col::TIMESTAMP)?)
}
