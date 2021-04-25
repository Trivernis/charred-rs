use crate::InputReader;
use crate::TapeResult;
use std::any::{Any, TypeId};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type TokenCheckerFn = Arc<
    dyn for<'a> Fn(
            &'a mut InputReader,
        ) -> Pin<Box<dyn Future<Output = TapeResult<Option<Token>>> + Send + 'a>>
        + Send
        + Sync,
>;

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
        self.inner_type_id() == TypeId::of::<T>()
    }

    /// Returns the TypeID of the inner stored type
    pub fn inner_type_id(&self) -> TypeId {
        self.inner.as_ref().type_id()
    }
}

/// Parsed when no other matching token was found for the character
pub struct UnknownToken(pub char);

/// End Of File Token
pub struct EOFToken;
