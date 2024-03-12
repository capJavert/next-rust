use serde_json::json;
use vercel_runtime::{Body, Error, Response, StatusCode};

pub fn throw_error(
    message: &str,
    error: Option<Error>,
    status_code: StatusCode,
) -> Result<Response<Body>, Error> {
    if let Some(error) = error {
        eprintln!("error: {error}");
    }

    Ok(Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(
            json!(
                {
                    "message": message
                }
            )
            .to_string()
            .into(),
        )?)
}

#[macro_export]
macro_rules! throw_error {
    ($message:expr, $error:expr) => {
        throw_error($message, $error, StatusCode::INTERNAL_SERVER_ERROR)
    };
    ($message:expr) => {
        throw_error($message, None, StatusCode::INTERNAL_SERVER_ERROR)
    };
    ($message:expr, $error:expr, $status_code:expr) => {
        throw_error($message, $error, $status_code)
    };
}
