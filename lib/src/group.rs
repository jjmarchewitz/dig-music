use clap::ValueEnum;

use std::fmt::Debug;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GroupType {
    Album,
    Artist,
    Episode,
    Podcast,
    Song,
}
