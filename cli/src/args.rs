use crate::spotify::SpotifyArgs;
use clap::{Parser, Subcommand};

// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// pub enum StreamingService {
//     Spotify,
//     Apple,
//     Amazon,
//     Google,
// }

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MainArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Spoopity
    #[command()]
    Spotify(SpotifyArgs),
    Apple,
    Amazon,
    Google,
}
