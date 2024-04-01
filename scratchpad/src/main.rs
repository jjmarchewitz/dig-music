#![allow(unused)]

use diglib;
use eyre::{Ok, Result};
use std::{path::PathBuf, str::FromStr};

const ETHAN_ZIP_PATH: &str =
    "/Users/jjmarch/Repos/dig-music/test-data/ethan_streaming_history_2023_OCT.zip";
const JAKE_ZIP_PATH: &str =
    "/Users/jjmarch/Repos/dig-music/test-data/jake_streaming_history_2023_SEP.zip";

fn main() -> Result<()> {
    let ethan_zip_path = PathBuf::from_str(ETHAN_ZIP_PATH)?;
    let jake_zip_path = PathBuf::from_str(JAKE_ZIP_PATH)?;

    let ethan_plays = diglib::load_plays(ethan_zip_path)?;
    let jake_plays = diglib::load_plays(jake_zip_path)?;

    Ok(())
}
