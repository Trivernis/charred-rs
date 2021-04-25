use crate::do_match;
use crate::matching::MatchResult;
use crate::tokenizing::{Token, TokenReader};
use std::any::TypeId;

/// Matches exactly one token
pub async fn match_one<T: 'static>(reader: &mut TokenReader) -> MatchResult<&Token> {
    let token = if reader.peek_is::<T>() {
        reader.consume()
    } else {
        return Ok(None);
    };

    Ok(Some(token))
}

/// Matches many tokens at once by TypeId
pub async fn match_many<'a, I: IntoIterator<Item = &'a TypeId>>(
    reader: &mut TokenReader,
    ids: I,
) -> MatchResult<&Token> {
    for id in ids {
        if &reader.peek().inner_type_id() == id {
            return Ok(Some(reader.consume()));
        }
    }

    Ok(None)
}

/// Matches many tokens at once by TypeId
pub async fn match_many_mul<'a, I: IntoIterator<Item = &'a TypeId> + Clone>(
    reader: &mut TokenReader,
    ids: I,
) -> MatchResult<usize> {
    let mut count = 0;
    while match_many(reader, ids.clone()).await?.is_some() {
        count += 1;
    }

    Ok(Some(count))
}

/// Matches many tokens at least once by TypeId
pub async fn match_many_mul_plus<'a, I: IntoIterator<Item = &'a TypeId> + Clone>(
    reader: &mut TokenReader,
    ids: I,
) -> MatchResult<usize> {
    let count = do_match!(match_many_mul(reader, ids));

    if count > 0 {
        Ok(Some(count))
    } else {
        Ok(None)
    }
}
