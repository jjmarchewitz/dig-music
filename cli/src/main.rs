mod args;
mod help;
mod spotify;

use args::{Commands, MainArgs};
use clap::Parser;
use eyre::Result;

fn main() -> Result<()> {
    let args = MainArgs::parse();

    match args.command {
        // Commands::Help => help::help_main(),
        Commands::Spotify(spotify_args) => spotify::spotify_main(spotify_args),
        _ => help::help_main(),
    }?;

    Ok(())
}
