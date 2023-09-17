use crate::Play;
use rayon::prelude::*;
use regex::Regex;
use serde_json;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use thiserror::Error;
use zip::ZipArchive;

pub fn load_plays(path_to_zip: PathBuf) -> Result<Vec<Play>, LoadError> {
    let mut archive = load_archive(&path_to_zip);
    let song_data = extract_song_data(&mut archive);
    let mut plays: Vec<Play> = Vec::new();

    // TODO: rewrite this as iterator
    for file_data in song_data.into_iter() {
        match serde_json::from_str::<Vec<Play>>(&file_data) {
            Ok(v) => plays.extend(v),
            Err(_) => return Err(LoadError::ParseError),
        }
    }

    Ok(plays)
}

fn load_archive<P: AsRef<Path>>(path: &P) -> ZipArchive<File> {
    let file = File::open(path).unwrap();
    ZipArchive::new(file).unwrap()
}

// TODO: Have multiple versions of this for different versions of the .zip that Spotify spits out
fn extract_song_data(archive: &mut ZipArchive<File>) -> Vec<String> {
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
}
