use diglib::{GroupType, PlayGroup};
use eyre::{Ok, Result};
use std::{collections::HashMap, path::PathBuf, str::FromStr};

const JAKE_ZIP_PATH: &str = "/Users/jjmarch/Repos/dig-music/test-data/find_song_I_liked.zip";

fn get_play_groups(
    path_str: &str,
    group_type: GroupType,
) -> Result<HashMap<String, Box<dyn PlayGroup>>> {
    let zip_path = PathBuf::from_str(path_str)?;
    let plays = diglib::load_plays(zip_path)?;
    let groups = diglib::group_plays_together(plays, group_type);

    Ok(groups.into_iter().map(|pg| (pg.get_hash(), pg)).collect())
}

pub fn run() -> Result<String> {
    let plays = get_play_groups(JAKE_ZIP_PATH, GroupType::Song)?;

    dbg!(plays);

    Ok("test".to_owned())
}
