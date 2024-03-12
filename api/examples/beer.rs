use next_rust::throw_error;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let response = reqwest::blocking::get("https://api.punkapi.com/v2/beers/random")?;
    let beers = response.json::<serde_json::Value>()?;

    let beer = match beers.get(0) {
        Some(beer) => beer,
        None => return Ok(throw_error!("no beer found", None, StatusCode::NOT_FOUND)?),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header(
            "Cache-Control",
            format!(
                "public, max-age=0, must-revalidate, s-maxage={s_maxage}",
                s_maxage = 24 * 60 * 60
            ),
        )
        .body(json!(beer).to_string().into())?)
}
