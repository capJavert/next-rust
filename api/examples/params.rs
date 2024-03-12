use std::collections::HashMap;

use serde_json::json;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // reading incoming req headers
    let headers = _req.headers();
    let user_agent = match headers.get("user-agent") {
        Some(value) => value.to_str().unwrap(),
        None => "unknown",
    };

    // parsing req url and path
    let url = Url::parse(&_req.uri().to_string()).unwrap();

    // read url query params
    let query_params = url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<String, String>>();

    // open /api/examples/params?q=test in your browser
    // and you will see the params in the response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({ 
              "path": url.path(), 
              "query": {
                "q": query_params.get("q")
              },
              "headers": {
              "userAgent": user_agent
            } })
            .to_string()
            .into(),
        )?)
}
