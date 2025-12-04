// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Name,
    Oldest,
    Newest,
}

#[derive(Debug, Clone)]
pub struct InvalidSortOrder(pub String);

impl std::fmt::Display for InvalidSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid sort order '{}'. Valid options: name, oldest, newest",
            self.0
        )
    }
}

impl std::error::Error for InvalidSortOrder {}

impl FromStr for SortOrder {
    type Err = InvalidSortOrder;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "name" => Ok(SortOrder::Name),
            "oldest" => Ok(SortOrder::Oldest),
            "newest" => Ok(SortOrder::Newest),
            _ => Err(InvalidSortOrder(s.to_string())),
        }
    }
}
