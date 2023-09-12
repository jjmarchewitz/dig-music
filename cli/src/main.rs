use std::path::PathBuf;

use deep_spotify::data;

fn main() {
    let path: PathBuf = "/Users/jjmarch/Repos/deep-spotify/test-data/my_spotify_data_JUL_23.zip"
        .parse()
        .unwrap();

    let plays = data::load_plays(path).unwrap();

    dbg!(plays);
}
