use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "env": dotenv::var("NODE_ENV").unwrap_or("development".to_string()),
              "vercelEnv": dotenv::var("VERCEL_ENV").unwrap_or("development".to_string()),
            })
            .to_string()
            .into(),
        )?)
}
