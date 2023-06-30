use crate::app_state::App;
use regex::Regex;
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};
use tauri::api::dialog;
use zip::{self, ZipArchive};

fn prompt_for_path() -> Option<PathBuf> {
    dialog::blocking::FileDialogBuilder::default()
        .add_filter("Zip Files", &["zip"])
        .pick_file()
}

fn load_archive<P: AsRef<Path>>(path: &P) -> ZipArchive<fs::File> {
    let file = fs::File::open(path).unwrap();
    zip::ZipArchive::new(file).unwrap()
}

fn extract_song_data(archive: &mut ZipArchive<fs::File>) -> Vec<String> {
    let re = Regex::new(r"endsong_[0-9]+\.json").unwrap();

    let song_data_file_names: Vec<String> = archive
        .file_names()
        .into_iter()
        .filter_map(|file_name| {
            if let Some(_) = re.find(file_name) {
                Some(file_name.to_string())
            } else {
                None
            }
        })
        .collect();

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
pub async fn prompt_for_spotify_files(app_state: tauri::State<'_, App>) -> Result<(), String> {
    // let mut app_state_with_lock = app_state.0.lock().unwrap();

    let path = prompt_for_path().unwrap();
    let mut archive = load_archive(&path);
    let song_data: Vec<String> = extract_song_data(&mut archive);

    let total_len: usize = song_data.iter().map(|data| data.len()).sum();

    dbg!(total_len);

    Ok(())
}
