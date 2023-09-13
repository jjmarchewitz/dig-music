use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyGenerateError {
    #[error("no album name exists for this item.")]
    MissingAlbumName,
    #[error("no artist name exists for this item.")]
    MissingArtistName,
    #[error("no podcast episode name exists for this item.")]
    MissingEpisodeName,
    #[error("no podcast name exists for this item.")]
    MissingPodcastName,
    #[error("no song name exists for this item.")]
    MissingSongName,
}
