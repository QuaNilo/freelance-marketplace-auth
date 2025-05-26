use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Params {
    user_id: i16,
    resource_type: String,
    resource_id: i16,
    action: String,
}

#[derive(Serialize)]
struct Response {
    authorized: bool
}

async fn check_authorization(Json(payload): Json<Params>) -> Json<Response> {
    let is_authorized = payload.user_id == 1 && payload.action.to_lowercase() == "edit";
    
    Json(Response {
        authorized: is_authorized,
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/authorization", get(check_authorization))
}