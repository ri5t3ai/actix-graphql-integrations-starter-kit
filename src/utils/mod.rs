use futures::Future;

use crate::{structs::utils::ReadJsonTreeSteps, structs::utils::ReadJsonTreeError};

pub fn read_json_tree<'a>(
    json_value: &'a serde_json::Value,
    path: &Vec<ReadJsonTreeSteps>,
) -> Result<&'a serde_json::Value, ReadJsonTreeError> {
    let mut actual_value: &serde_json::Value = json_value;

    for step in path {
        match step {
            ReadJsonTreeSteps::Key(key) => {
                actual_value = actual_value
                    .get(key)
                    .ok_or(ReadJsonTreeError::JsonNotFound)?;
            }
            ReadJsonTreeSteps::Index(index) => {
                actual_value = actual_value
                    .get(*index)
                    .ok_or(ReadJsonTreeError::JsonNotFound)?;
            }
        }
    }

    Ok(actual_value)
}

/// Retry function for async functions
/// # Example
/// ```
/// use shopify_api::utils::retry_async;
/// use std::io::{Error, ErrorKind};
///
/// async fn my_async_function(args: &(String, u8)) -> Result<(), Error> {
///    Err(Error::new(ErrorKind::Other, "Error"))
/// }
///
/// #[tokio::main]
/// async fn main() {
///   let result = retry_async(3, my_async_function, &(String::from("test"), 1)).await;
///  assert!(result.is_err());
/// }
/// ```
/// # Errors
/// This function returns an error if the async function returns an error
/// # Panics
/// This function panics if the number of retries is 0
pub async fn retry_async<'a, Fut, F, Args, Out, ErrOut>(
    max_retries: u64,
    func: Fut,
    args: &'a Args,
) -> Result<Out, ErrOut>
where
    Fut: Fn(&'a Args) -> F,
    F: Future<Output = Result<Out, ErrOut>>,
    ErrOut: std::fmt::Debug,
{
    if max_retries == 0 {
        panic!("Max retries cannot be 0");
    }

    let mut count: u64 = 0;
    let mut result: Result<Out, ErrOut> = func(args).await;

    while count < max_retries - 1 && result.is_err() {
        let executed_func = func(args);

        result = executed_func.await;
        if result.is_ok() {
            return result;
        }

        count += 1;
    }

    result
}