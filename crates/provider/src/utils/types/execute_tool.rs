use super::{
    super::{
        is_async_iterable::IsAsyncIterable,
        types::tool::ToolExecutionOptions,
    },
};

use async_stream::stream;
use futures::{Stream, StreamExt};
use std::future::Future;

#[derive(Clone, Debug)]
pub enum ToolOutput<T> {
    Preliminary(T),
    Final(T),
}

pub fn execute_tool<Input, Output, F, Fut, E>(
    execute: F,
    input: Input,
    options: ToolExecutionOptions,
) -> impl Stream<Item = Result<ToolOutput<Output>, E>>
where
    Input: Send,
    Output: Clone + Send + 'static,
    E: Send + 'static,
    F: FnOnce(Input, ToolExecutionOptions) -> Fut,
    Fut: Future<Output = Result<IsAsyncIterable<Output>, E>> + Send,
{
    stream! {
        let result = execute(input, options).await;

        match result {
            Ok(IsAsyncIterable::Stream(mut s)) => {
                let mut last_output = None;
                while let Some(output) = s.next().await {
                    last_output = Some(output.clone());
                    yield Ok(ToolOutput::Preliminary(output));
                }
                if let Some(last) = last_output {
                    yield Ok(ToolOutput::Final(last));
                }
            }
            Ok(IsAsyncIterable::Value(val)) => {
                yield Ok(ToolOutput::Final(val));
            }
            Err(e) => {
                yield Err(e);
            }
        }
    }
}
