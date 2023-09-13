use crate::aggregate::AggregatedData;

mod album;
mod artist;
mod episode;
mod podcast;
mod song;

pub use album::Album;
pub use artist::Artist;
pub use episode::Episode;
pub use podcast::Podcast;
pub use song::Song;

pub trait PlayGroup {
    fn get_aggregated_data(&mut self) -> &AggregatedData;
    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData;

    fn key_string(&self) -> String;
}
