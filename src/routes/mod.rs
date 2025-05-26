pub mod auth;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(auth::router())
}