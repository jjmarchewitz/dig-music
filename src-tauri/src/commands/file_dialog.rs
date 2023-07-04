use crate::{app_state::App, data::Play};
use rayon::prelude::*;
use regex::Regex;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use tauri::api::dialog::blocking::FileDialogBuilder;
use zip::ZipArchive;

fn prompt_for_path() -> Option<PathBuf> {
    FileDialogBuilder::default()
        .add_filter("Zip Files", &["zip"])
        .pick_file()
}

fn load_archive<P: AsRef<Path>>(path: &P) -> ZipArchive<File> {
    let file = File::open(path).unwrap();
    ZipArchive::new(file).unwrap()
}

fn extract_song_data(archive: &mut ZipArchive<File>) -> Vec<String> {
    let re = Regex::new(r"endsong_[0-9]+\.json").unwrap();

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

// TODO: Change error type to custom one
#[tauri::command]
pub async fn load_user_data(app_state: tauri::State<'_, App>) -> Result<(), String> {
    let path = prompt_for_path().unwrap();
    let mut archive = load_archive(&path);
    let song_data = extract_song_data(&mut archive);

    let mut app_state_with_lock = app_state.0.lock().unwrap();

    app_state_with_lock.plays = song_data
        .into_par_iter()
        .flat_map(|file_data| {
            let d: Vec<Play> = serde_json::from_str::<Vec<Play>>(&file_data).unwrap();
            d.into_par_iter()
        })
        .collect();

    Ok(())
}
