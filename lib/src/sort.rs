use std::fmt::Display;

use clap::ValueEnum;

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
