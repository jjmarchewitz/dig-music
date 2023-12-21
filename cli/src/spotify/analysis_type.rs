use clap::ValueEnum;
use dig_music_lib::GroupType;
use thiserror::Error;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AnalysisType {
    Meta,
    Album,
    Artist,
    Episode,
    Podcast,
    Song,
}

#[derive(Error, Debug)]
#[error("Cannot convert the \"meta\" analysis type to a group type")]
pub struct CannotConvertMetaToGroup;

impl TryInto<GroupType> for AnalysisType {
    type Error = CannotConvertMetaToGroup;

    fn try_into(self) -> Result<GroupType, Self::Error> {
        match self {
            Self::Meta => Err(CannotConvertMetaToGroup),
            Self::Album => Ok(GroupType::Album),
            Self::Artist => Ok(GroupType::Artist),
            Self::Episode => Ok(GroupType::Episode),
            Self::Podcast => Ok(GroupType::Podcast),
            Self::Song => Ok(GroupType::Song),
        }
    }
}
