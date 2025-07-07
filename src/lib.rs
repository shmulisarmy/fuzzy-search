//! Fuzzy search library
//! 
//! Provides a simple fuzzy searcher for matching substrings in a list of strings.

mod search;
pub use search::{FuzzySearcher, BeingSearchedState};

#[cfg(test)]
mod tests; 