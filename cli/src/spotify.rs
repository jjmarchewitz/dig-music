use clap::Args;
use eyre::Result;
use rayon::prelude::*;

use dig_music_lib::{GroupType, PlayGroup, SortOrder};

#[derive(Args, Debug)]
pub struct SpotifyArgs {
    /// The path to the .zip file that Spotify gave you. This MUST be for extended listen history ONLY. Don't use the .zip for your general account data.
    pub path: String,

    /// How you want your listen history to be grouped together.
    pub group_type: GroupType,

    /// The ordering of your results
    #[arg(short, long, value_enum, default_value_t = SortOrder::Descending)]
    pub sort: SortOrder,

    /// The maximum number of results to show.
    #[arg(long)]
    pub limit: Option<usize>,
}

pub fn spotify_main(args: SpotifyArgs) -> Result<()> {
    // TODO: define error type for load_plays
    let plays = dig_music_lib::load_plays(args.path.into())?;

    // TODO: Turn this into a Vec<PlayGroup> where PlayGroup is an enum
    let mut grouped_data = dig_music_lib::group_plays_together(plays, GroupType::Artist);

    grouped_data.par_sort_by_key(|e| e.get_aggregated_data().ms_played.total);

    let grouped_data: Vec<(usize, Box<dyn PlayGroup>)> = {
        let ranks_iterator = 1..(grouped_data.len() + 1);
        let zipped_iterator = ranks_iterator.rev().zip(grouped_data.into_iter());

        match args.sort {
            SortOrder::Ascending => zipped_iterator.collect(),
            SortOrder::Descending => zipped_iterator.rev().collect(),
        }
    };

    for (rank, group) in grouped_data.iter() {
        let rank_str = format!("{}.", rank);
        println!("{:7}{}", rank_str, group.get_metadata().to_string());
        println!(
            "Time: {}, Plays: {}\n",
            group.get_aggregated_data().display_ms_played(),
            group.get_aggregated_data().play_count
        );

        if let Some(limit) = args.limit {
            if *rank >= limit {
                break;
            }
        }
    }

    Ok(())
}
