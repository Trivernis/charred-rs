use crate::error::TapeResult;
use crate::input_reader::InputReader;
use async_trait::async_trait;
use std::any::{Any, TypeId};

#[async_trait]
pub trait ProtoToken {
    /// Tries parsing the token
    async fn try_parse(reader: &mut InputReader) -> TapeResult<Option<Token>>;
}

pub struct Token {
    inner: Box<dyn Any>,
}

impl Token {
    /// Constructs a new token
    pub fn new<A: Any>(inner: A) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    /// Tries downcasting the value to a concrete type
    pub fn try_as<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref::<T>()
    }

    pub fn try_into<T: 'static>(self) -> Option<T> {
        match self.inner.downcast() {
            Ok(value) => Some(*value),
            Err(_) => None,
        }
    }

    /// Checks if the inner value is of a given concrete type
    pub fn is<T: 'static>(&self) -> bool {
        self.inner.as_ref().type_id() == TypeId::of::<T>()
    }
}
