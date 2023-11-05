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
    let mut archive = load_archive(&path_to_zip)?;
    let song_data = extract_song_data(&mut archive);
    let mut plays: Vec<Play> = Vec::new();

    // TODO: rewrite this as iterator
    for file_data in song_data.into_iter() {
        match serde_json::from_str::<Vec<Play>>(&file_data) {
            Ok(v) => plays.extend(v),
            Err(_) => return Err(LoadError::ParseError),
        }
    }

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

    df = df
        .lazy()
        .with_columns([col("ts").cast(DataType::Datetime(TimeUnit::Microseconds, None))])
        .collect()?;

    rename_columns(&mut df)?;

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
fn extract_song_data(archive: &mut zip::ZipArchive<File>) -> Vec<String> {
    let re = Regex::new(r"Streaming_History_Audio_.*\.json").unwrap();

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

    // TODO: rewrite this block based on this: https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
    // This cannot be made parallel because we can't have concurrent access to a single mutable reference.
    // In order to be parallel, the archive would need to have its contents read from multiple places at once.
    let file_contents: Vec<String> = song_data_file_names
        .into_iter()
        .map(|file_name| {
            let mut file = archive.by_name(&file_name).unwrap();

            let mut content = String::default();

            file.read_to_string(&mut content).unwrap();

            content
        })
        .collect();

    file_contents
}

fn rename_columns(df: &mut DataFrame) -> PolarsResult<&mut DataFrame> {
    Ok(df
        .rename("master_metadata_album_album_name", "album_name")?
        .rename("master_metadata_album_artist_name", "artist_name")?
        .rename("master_metadata_track_name", "track_name")?
        .rename("conn_country", "connected_from_country")?
        .rename("episode_show_name", "podcast_name")?
        .rename("ts", "timestamp")?)
}
