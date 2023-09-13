use std::path::PathBuf;

use dig_music;

fn main() {
    let path: PathBuf = "/Users/jjmarch/Repos/dig-music/test-data/my_spotify_data_JUL_23.zip"
        .parse()
        .unwrap();

    let plays = dig_music::load_plays(path).unwrap();

    dbg!(plays);
}
