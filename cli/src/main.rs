use std::path::PathBuf;

use dig_music::{self, GroupType};

use rayon::prelude::*;

fn main() {
    let path: PathBuf = "/Users/jjmarch/Repos/dig-music/test-data/my_spotify_data_JUL_23.zip"
        .parse()
        .unwrap();

    let plays = dig_music::load_plays(path).unwrap();

    // TODO: Turn this into a Vec<PlayGroup> where PlayGroup is an enum
    let mut grouped_data = dig_music::group_plays_together(plays, GroupType::Artist);

    grouped_data.par_sort_by_key(|e| e.get_aggregated_data().ms_played.total);

    dbg!(grouped_data);
}
