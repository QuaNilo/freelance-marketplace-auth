use axum::{routing::get, Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::config::{Mongo, Settings};
use crate::db::mongo::MongoClient;
use crate::db::postgres::{PostgresClient};
use crate::models::route_logic::Route;
use crate::models::sql::user::User;
use crate::models::no_sql::notification;
use crate::models::no_sql::notification::Notification;
use crate::utils::auth_utils::{is_nosql_resource, is_sql_resource};

#[derive(Deserialize)]
struct ResourceAuthorizationParams {
    user_id: i32,
    resource_type: String,
    resource_id: i32,
    action: String,
}
#[derive(Deserialize)]
struct RouteAuthorizationParams {
    route: String,
    user_id: i32,
}

#[derive(Serialize)]
struct Response {
    authorized: bool
}

async fn check_authorization(Json(payload): Json<ResourceAuthorizationParams>) -> Json<Response> {
    assert!(is_nosql_resource(&payload.resource_type).await || is_sql_resource(&payload.resource_type).await);
    let settings = Settings::new();
    let postgres = PostgresClient::new(&settings.sql.connection_string).await.expect("Failed to connect to Postgres");
    let mongo: MongoClient = MongoClient::new(&settings.mongo.connection_string, &settings.mongo.database_name).await.expect("Failed to connect to MongoDB");
    
    let user: Option<User> = postgres.get_item_by_id(
        &payload.user_id,
        "SELECT * FROM users WHERE user_id = $1"
    ).await.unwrap_or_else(|e|{
        eprintln!("Database error: {:?}", e);
        None
    });
    
    if let Some(user) = user {
        if user.is_deleted().await {
            return Json(Response{authorized: false})
        }
    }
    
    Json(Response {
        authorized: true,
    })
}

async fn check_route_authorization(Json(payload): Json<RouteAuthorizationParams>) -> Json<Response> {
    let settings = Settings::new();
    let postgres = PostgresClient::new(&settings.sql.connection_string).await.expect("Failed to connect to Postgres");
    
    let query: &str = "SELECT * FROM users JOIN roles ON users.role_id = roles.role_id WHERE users.user_id = $1";
    let user: Option<User> = postgres.get_item_by_id(
        &payload.user_id,
        query
    ).await.unwrap_or_else(|e|{
        eprintln!("Database error: {:?}", e);
        None
    });
    let all_routes: Vec<Route> = Route::get_routes().await;
    let valid = all_routes.iter().any(|route|  route.path == payload.route);
    if !valid {
        return Json(Response{authorized: false});
    }
        
    let route : Option<&Route> = all_routes.iter().find(|route| route.path == payload.route);
    if let Some(route) = route {
        if !route.is_private {
            return Json(Response{authorized: true});
        }
    }
    
    
    
    Json(
        Response {authorized: true}
    )
    
}

#[derive(Deserialize)]
struct ResourceParams {
    notification_id: String,
}
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn get_notification(Json(payload): Json<ResourceParams>) -> Result<Json<Notification>, (StatusCode, Json<ErrorResponse>)> {
    let notification = Notification::get_notification(&payload.notification_id).await;
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
        .route("/authorization/route", get(check_route_authorization))
}