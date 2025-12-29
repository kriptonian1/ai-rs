use futures::stream::Stream;
use std::pin::Pin;

pub enum IsAsyncIterable<T> {
    Value(T),
    Stream(Pin<Box<dyn Stream<Item = T> + Send>>),
}

impl<T> IsAsyncIterable<T> {
    pub fn is_async_iterable(&self) -> bool {
        matches!(self, IsAsyncIterable::Stream(_))
    }
}