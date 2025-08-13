use gloo_net::http::{Request, Response};
use web_sys::RequestCredentials;

pub async fn post_json(url: &str, payload: &impl serde::Serialize) -> Result<Response, String> {
    let result = Request::post(url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .json(payload);

    match result {
        Ok(req) => req
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e)),
        Err(e) => return Err(format!("Request failed: {}", e)),
    }
}

pub async fn put_json(url: &str, payload: &impl serde::Serialize) -> Result<Response, String> {
    let result = Request::put(url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .json(payload);

    match result {
        Ok(req) => req
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e)),
        Err(e) => return Err(format!("Request failed: {}", e)),
    }
}

pub async fn fetch_data(url: &str) -> Result<Response, String> {
    Request::get(url)
        .credentials(RequestCredentials::Include)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))
}
