use std::collections::HashMap;

use next_rust::throw_error;
use serde_json::json;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&_req.uri().to_string()).unwrap();
    let query_params = url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<String, String>>();

    let q = match query_params.get("q") {
        Some(value) => value,
        None => "",
    };

    if q.is_empty() {
        return Ok(throw_error!(
            "query param 'q' is required",
            None,
            StatusCode::BAD_REQUEST
        )?);
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
             "message": format!("You searched for '{q}'", q=q)
            })
            .to_string()
            .into(),
        )?)
}
