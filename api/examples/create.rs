use next_rust::throw_error;
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    name: String,
}

fn route_post(_req: Request) -> Result<Response<Body>, Error> {
    let body: CreatePayload = match serde_json::from_slice(_req.body()) {
        Ok(body) => body,
        Err(_) => {
            return Ok(throw_error!(
                "invalid request body",
                None,
                StatusCode::BAD_REQUEST
            )?);
        }
    };

    if body.name.trim().is_empty() {
        return Ok(throw_error!(
            "name is required",
            None,
            StatusCode::BAD_REQUEST
        )?);
    }

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(
            json!({ "message": format!("created {name}!", name=body.name) })
                .to_string()
                .into(),
        )?)
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let response = match _req.method().to_owned() {
        Method::POST => route_post(_req),
        _ => {
            return Ok(throw_error!(
                "method not allowed",
                None,
                StatusCode::METHOD_NOT_ALLOWED
            )?);
        }
    };

    return response;
}
