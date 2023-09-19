use std::path::PathBuf;

use clap::Args;
use eyre::Result;
// use rayon::prelude::*;

use dig_music_lib::{GroupType, PlayGroup, SortBy, SortOrder};

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

    /// Path to create a XLSX file at
    #[arg(long)]
    pub xlsx: Option<PathBuf>,
}

pub fn spotify_main(args: SpotifyArgs) -> Result<()> {
    let plays = dig_music_lib::load_plays(args.path)?;

    let grouped_data = dig_music_lib::group_plays_together(plays, args.group_type);

    let sorted_data = dig_music_lib::sort_data(grouped_data, args.sort, args.order);

    print_data(sorted_data, args.limit);

    Ok(())
}

fn print_data(data: Vec<(usize, Box<dyn PlayGroup>)>, limit: Option<usize>) {
    for (index, (rank, group)) in data.iter().enumerate() {
        let rank_str = format!("{}.", rank);
        println!("\n{} {}", rank_str, group.get_metadata().to_string());
        println!(
            "Plays: {}, Time: {}",
            group.get_aggregated_data().play_count,
            group.get_aggregated_data().display_ms_played(),
        );

        if let Some(limit) = limit {
            if limit == 0 || index >= limit - 1 {
                break;
            }
        }
    }
}
