use std::path::PathBuf;

use clap::Args;
use eyre::Result;
use polars::prelude::*;
use std::fs::File;

use dig_music_lib::{GroupType, SortBy, SortOrder};

// TODO: exporting to playlist (track URIs)
#[derive(Args, Debug)]
pub struct SpotifyArgs {
    /// The path to the .zip file that Spotify gave you. This MUST be for extended listen history ONLY. Don't use the .zip for your general account data.
    pub path: PathBuf,

    /// How you want your listen history to be grouped together.
    pub group_type: GroupType,

    /// How to sort the results
    #[arg(short, long, value_enum, default_value_t = SortBy::Time)]
    pub sort: SortBy,

    /// The ordering of your results
    #[arg(short, long, value_enum, default_value_t = SortOrder::Descending)]
    pub order: SortOrder,

    /// The maximum number of results to show.
    #[arg(long)]
    pub limit: Option<usize>,

    /// Path to create a CSV file at
    #[arg(long)]
    pub csv: Option<PathBuf>,
}

// TODO: filter-plays
// TODO: filter-results
// /// Path to create a XLSX file at
// #[arg(long)]
// pub xlsx: Option<PathBuf>,

pub fn spotify_main(args: SpotifyArgs) -> Result<()> {
    let mut df = dig_music_lib::load_plays_to_df(args.path)?;

    // let plays = dig_music_lib::load_plays(args.path)?;
    // let grouped_data = dig_music_lib::group_plays_together(plays, args.group_type);
    // let sorted_data = dig_music_lib::sort_data(grouped_data, args.sort, args.order);

    // print_data(sorted_data, args.limit);

    if let Some(csv_path) = args.csv {
        let mut file = File::create(csv_path).expect("could not create CSV file");

        CsvWriter::new(&mut file)
            .has_header(true)
            .with_delimiter(b',')
            .finish(&mut df)?;
    }

    Ok(())
}
