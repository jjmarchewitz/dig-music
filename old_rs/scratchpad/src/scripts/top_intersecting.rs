use diglib::{GroupType, Play, PlayGroup};
use eyre::Result;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

const ETHAN_ZIP_PATH: &str =
    "/Users/jjmarch/Repos/dig-music/test-data/ethan_streaming_history_2023_OCT.zip";
const JAKE_ZIP_PATH: &str =
    "/Users/jjmarch/Repos/dig-music/test-data/jake_streaming_history_2023_SEP.zip";

fn get_play_groups(
    path_str: &str,
    group_type: GroupType,
) -> Result<HashMap<String, Box<dyn PlayGroup>>> {
    let zip_path = PathBuf::from_str(path_str)?;
    let plays = diglib::load_plays(zip_path)?;
    let plays = filter_plays(plays);
    let groups = diglib::group_plays_together(plays, group_type);

    Ok(groups.into_iter().map(|pg| (pg.get_hash(), pg)).collect())
}

fn filter_plays(plays: Vec<Play>) -> Vec<Play> {
    plays
        .into_iter()
        .filter(|p| {
            let Some(ms_played) = p.ms_played else {
                return false;
            };

            ms_played > (60 * 1000)
        })
        .collect()
}

fn filter_play_groups(
    hm: HashMap<String, Box<dyn PlayGroup>>,
) -> HashMap<String, Box<dyn PlayGroup>> {
    let min_play_count: u64 = 25;

    hm.into_iter()
        .filter(|(_, pg)| pg.get_agg_data().play_count >= min_play_count)
        .collect()
}

pub fn run() -> Result<String> {
    let ethan_songs = get_play_groups(ETHAN_ZIP_PATH, GroupType::Song)?;
    let jake_songs = get_play_groups(JAKE_ZIP_PATH, GroupType::Song)?;

    let mut ethan_songs = filter_play_groups(ethan_songs);
    let mut jake_songs = filter_play_groups(jake_songs);

    let ethan_keys: HashSet<String> = ethan_songs.keys().cloned().collect();
    let jake_keys: HashSet<String> = jake_songs.keys().cloned().collect();

    let intersection_keys: HashSet<String> = ethan_keys.intersection(&jake_keys).cloned().collect();

    let mut intersection_songs: Vec<Box<dyn PlayGroup>> = intersection_keys
        .into_iter()
        .map(|key| {
            let ethan_song = ethan_songs
                .remove(&key)
                .expect("Didn't find key in Ethan's songs");

            let jake_song = jake_songs
                .remove(&key)
                .expect("Didn't find key in Jake's songs");

            let mut combined_song = ethan_song;
            *combined_song.get_agg_data_mut() += jake_song.get_agg_data().clone();

            combined_song
        })
        .collect();

    intersection_songs.sort_by_key(|pg| pg.get_agg_data().play_count);
    intersection_songs.reverse();

    let playlist_str = intersection_songs
        .into_iter()
        .filter_map(|pg| pg.get_agg_data().track_uri.clone())
        .collect::<Vec<String>>()
        .join("\n");

    Ok(playlist_str)
}
