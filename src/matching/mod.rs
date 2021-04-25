use crate::TapeResult;

mod matchers;

pub type MatchResult<T> = TapeResult<Option<T>>;
pub use matchers::*;

#[macro_export]
macro_rules! do_match {
    ($matcher:expr) => {
        match $matcher.await? {
            Some(inner) => inner,
            None => return Ok(None),
        }
    };
}
