use axum::{routing::get, Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::utils::notifications;
use crate::utils::notifications::NotificationsSchema;

#[derive(Deserialize)]
struct AuthorizationParams {
    user_id: i16,
    resource_type: String,
    resource_id: i16,
    action: String,
}

#[derive(Serialize)]
struct Response {
    authorized: bool
}

async fn check_authorization(Json(payload): Json<AuthorizationParams>) -> Json<Response> {
    let is_authorized = payload.user_id == 1 && payload.action.to_lowercase() == "edit";
    
    Json(Response {
        authorized: is_authorized,
    })
}

#[derive(Deserialize)]
struct ResourceParams {
    notification_id: String,
}
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn get_notification(Json(payload): Json<ResourceParams>) -> Result<Json<NotificationsSchema>, (StatusCode, Json<ErrorResponse>)> {
    let notification = notifications::get_notification(&payload.notification_id).await;
    match notification {
        Ok(notification) => Ok(Json(notification)),
        Err(e) => {
            let error_msg = e.to_string();
             Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse { error: error_msg }),
            ))
        },
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/authorization", get(check_authorization))
        .route("/notification", get(get_notification))
}