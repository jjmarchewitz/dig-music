mod analysis_type;

use analysis_type::AnalysisType;
use clap::Args;
use dig_music_lib::{SortBy, SortOrder};
use eyre::Result;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct SpotifyArgs {
    /// The path to the .zip file that Spotify gave you. This MUST be for extended listen history ONLY. Don't use the .zip for your general account data.
    pub path: PathBuf,

    /// How you want your listen history to be analyzed together (songs, albums, podcasts, or a meta-analysis).
    pub analysis_type: AnalysisType,

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
    // jake mode: every combo of group/sort-by
    // "meta" in addition to "song", "album", etc. meta statistics about your account
    // (i.e. total listen time overall).

    // TODO: export result DF to a playlist (track URIs)
    // use keyring, oauth2 crates and ask user to authenticate if they haven't already
    #[arg(long)]
    pub create_playlist: Option<String>,
}

// TODO: filter-plays
// TODO: filter-results

pub fn spotify_main(args: SpotifyArgs) -> Result<()> {
    let df = dig_music_lib::load_plays_to_df(args.path)?;

    // If performing meta analysis, print the analysis and immediately terminate
    if let AnalysisType::Meta = args.analysis_type {
        dig_music_lib::print_meta_analysis(df);
        return Ok(());
    }

    let mut df = dig_music_lib::group_plays(df, args.analysis_type.try_into()?)?;

    dbg!(df.head(Some(2)).get_columns());

    // let grouped_data = dig_music_lib::group_plays_together(plays, args.group_type);
    // let sorted_data = dig_music_lib::sort_data(grouped_data, args.sort, args.order);
    // print_data(sorted_data, args.limit);

    // if let Some(csv_path) = args.csv {
    //     // TODO: prep for CSV function
    //     dig_music_lib::write_df_to_csv(&mut df, &csv_path)?;
    // }

    Ok(())
}
