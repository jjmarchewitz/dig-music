use crate::PlayGroup;
use clap::ValueEnum;
use rayon::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Descending
    }
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => f.write_str("Ascending"),
            SortOrder::Descending => f.write_str("Descending"),
        }
    }
}

// impl Display for Option<SortOrder> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Some(d) => d.fmt(f),
//             Self::None => f.write_str("NONE"),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortBy {
    Plays,
    Time,
}

pub fn sort_data(
    grouped_data: Vec<Box<dyn PlayGroup>>,
    sort: SortBy,
    order: SortOrder,
) -> Vec<(usize, Box<dyn PlayGroup>)> {
    let ranks_iterator = 1..(grouped_data.len() + 1);
    let zipped_iterator = ranks_iterator.rev().zip(grouped_data.into_iter());

    match order {
        SortOrder::Ascending => zipped_iterator.collect(),
        SortOrder::Descending => zipped_iterator.rev().collect(),
    }
}

fn get_sort_fn(sort: SortBy) -> impl Fn(Box<dyn PlayGroup>) -> Box<dyn Ord> {
    match sort {
        SortBy::Plays => |e: Box<dyn PlayGroup>| Box::new(e.get_aggregated_data().play_count),
        SortBy::Time => todo!(),
    }
}