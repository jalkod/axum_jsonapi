use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::json_api::Output;

/// Input query object
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct NumQuery {
    n: usize,
}

/// Return object
#[derive(Serialize)]
struct NumQueryResponse {
    n: usize,
    result: usize,
}

/// Recursive, un-memoised Fibonacci, to simulate long work
fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Async, non-blocking query processor to calculate the n-th Fibonacci number
///
/// Usage example: /fib?n=20
pub(crate) async fn fib(Query(NumQuery { n }): Query<NumQuery>) -> impl IntoResponse {
    let mut result = Output::<NumQueryResponse>::new();
    if n > 40 {
        result
            .add_error(StatusCode::NOT_ACCEPTABLE)
            .err_detail(format!("Number is too high: {n}"))
            .err_source("/fib".into());
        return result.respond();
    }

    let fib = tokio::task::spawn_blocking(move || fibonacci(n)).await;
    match fib {
        Ok(res) => result.add(NumQueryResponse { n, result: res }),
        Err(err) => result
            .add_error(StatusCode::UNPROCESSABLE_ENTITY)
            .err_detail(format!("Calculation failed: {err}"))
            .err_source("fib".into()),
    };
    result.respond()
}

/// Errors endpoint
///
/// JSON:API defines an error list of a specific format. Here we create such list and return it in the expected format.
pub(crate) async fn errors() -> impl IntoResponse {
    let mut result = Output::<u8>::new();
    result
        .add_error(StatusCode::SERVICE_UNAVAILABLE)
        .err_source("/errors".into())
        .err_detail("First error".into())
        .add_error(StatusCode::IM_A_TEAPOT)
        .err_detail("Second, teapot error".into());
    result.respond()
}

#[cfg(test)]
mod test_fibonacci {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(20), 6765);
    }
}
