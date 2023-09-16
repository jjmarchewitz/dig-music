// use std::path::PathBuf;

use dig_music::{self, GroupType};

use clap::{Parser, Subcommand, ValueEnum};
use eyre::Result;
// use rayon::prelude::*;

mod help;
mod spot;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum StreamingService {
    Spotify,
    Apple,
    Amazon,
    Google,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Spotify { path: String },
    Apple { path: String },
    Amazon { path: String },
    Google { path: String },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        // Commands::Help => help::help_main(),
        Commands::Spotify { path } => spot::spotify_main(path),
        _ => help::help_main(),
    }

    // let plays = dig_music::load_plays(args.path.into()).unwrap();

    // TODO: Turn this into a Vec<PlayGroup> where PlayGroup is an enum
    // let mut grouped_data = dig_music::group_plays_together(plays, GroupType::Artist);

    // grouped_data.par_sort_by_key(|e| e.get_aggregated_data().ms_played.total);

    // dbg!(grouped_data);

    Ok(())
}
