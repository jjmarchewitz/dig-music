use crate::Play;
use ::zip;
use polars::prelude::*;
use rayon::prelude::*;
use regex::Regex;
use std::{
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};
use thiserror::Error;

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

    Ok(df)
}

// pub fn load_plays_to_df(path_to_zip: PathBuf) -> Result<DataFrame, LoadError> {
//     let mut archive = load_archive(&path_to_zip)?;
//     let song_data = extract_song_data(&mut archive);

//     let schema = Schema::from_iter(vec![
//         Field::new("conn_country", DataType::Utf8),
//         Field::new("episode_name", DataType::Utf8),
//         Field::new("episode_show_name", DataType::Utf8),
//         Field::new("incognito_mode", DataType::Boolean),
//         Field::new("ip_addr_decrypted", DataType::Utf8),
//         Field::new("master_metadata_album_album_name", DataType::Utf8),
//         Field::new("master_metadata_album_artist_name", DataType::Utf8),
//         Field::new("master_metadata_track_name", DataType::Utf8),
//         Field::new("ms_played", DataType::UInt64),
//         Field::new("offline", DataType::Boolean),
//         Field::new("offline_timestamp", DataType::UInt64),
//         Field::new("platform", DataType::Utf8),
//         Field::new("reason_end", DataType::Utf8),
//         Field::new("reason_start", DataType::Utf8),
//         Field::new("shuffle", DataType::Boolean),
//         Field::new("skipped", DataType::Boolean),
//         Field::new("spotify_episode_uri", DataType::Utf8),
//         Field::new("spotify_track_uri", DataType::Utf8),
//         Field::new("ts", DataType::Datetime(TimeUnit::Microseconds, None)),
//         Field::new("user_agent_decrypted", DataType::Utf8),
//         Field::new("username", DataType::Utf8),
//     ]);

//     let mut all_plays_df: Option<DataFrame> = None;

//     for s in song_data.into_iter() {
//         let cursor = Cursor::new(s);
//         let new_df = JsonReader::new(cursor)
//             .with_schema(schema.clone().into())
//             .finish()?;

//         if let Some(df) = &mut all_plays_df {
//             *df = df.hstack(new_df.get_columns())?;
//         } else {
//             all_plays_df = Some(new_df)
//         }
//     }

//     if let Some(df) = all_plays_df {
//         Ok(df)
//     } else {
//         Err(LoadError::FailedToCollectData)
//     }
// }

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

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("unable to parse your data.")]
    ParseError,

    #[error("unable to open the file at the given path.")]
    UnableToOpenFile,

    #[error("unable to load the data from the .zip file at the given path.")]
    UnableToLoadZipData,

    #[error("unable to construct a DataFrame from the given data.")]
    CannotConstructDataframe(#[from] PolarsError),

    #[error("collecting data from the .zip into a DataFrame failed.")]
    FailedToCollectData,
}
