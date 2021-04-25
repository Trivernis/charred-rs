#[macro_use]
pub mod matching;
pub mod tokenizing;
mod utils;

pub use utils::error::*;
pub use utils::input_reader::InputReader;

#[cfg(test)]
mod tests;
