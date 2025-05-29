use axum::{routing::get, Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::config::{Mongo, Settings};
use crate::db::mongo::{MongoClient};
use crate::db::postgres::{PostgresClient};
use crate::models::no_sql::message::Message;
use crate::models::route_logic::Route;
use crate::models::sql::user::User;
use crate::models::no_sql::notification;
use crate::models::no_sql::notification::Notification;
use crate::models::no_sql::portfolio::Portfolio;
use crate::models::no_sql::conversation::Conversation;
use crate::models::no_sql::wishlist::WishList;
use crate::models::sql::category::Category;
use crate::models::sql::milestone::Milestone;
use crate::models::sql::order::Order;
use crate::models::sql::profile::Profile;
use crate::models::sql::proposal::Proposal;
use crate::models::sql::request::Request;
use crate::models::sql::review::Review;
use crate::models::sql::role::Role;
use crate::models::sql::service::Service;
use crate::models::sql::skill::Skill;
use crate::models::sql::sub_category::SubCategory;
use crate::models::sql::transaction::Transaction;
use crate::models::sql::wallet_types::WalletType;
use crate::traits::fetchable_resource;
use crate::traits::fetchable_resource::{DbClients, FetchableResource};
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
    let db_clients = DbClients { postgres, mongo };
    
    let request_user: User = match db_clients.postgres.get_item_by_id(
        &payload.user_id,
        "SELECT * FROM users WHERE user_id = $1"
    ).await.unwrap_or_else(|e|{
        eprintln!("Database error: {:?}", e);
        None
    }) {
        Some(request_user) => request_user,
        None => return Json(Response{authorized: false})
    };
    
    if request_user.is_deleted() {
        return Json(Response{authorized: false})
    }
    
    if let Some(resource) = fetch_resource_by_type(&db_clients, &payload.resource_type, &payload.resource_id.to_string()).await {
        let is_authorized = match resource {
            // SQL
            Resource::User(user) => {
                if user.is_deleted() { false }
                else if  request_user.user_id != user.user_id { false }
                else { true }
            }
            // TODO Implement the remaining resource permissions
            Resource::SubCategory(sub_category) => {false}
            Resource::Category(category) => {false}
            Resource::Transaction(transaction) => {false}
            Resource::Order(order) => {false}
            Resource::Proposal(proposal) => {false}
            Resource::Milestone(milestone) => {false}
            Resource::Service(service) => {false}
            Resource::Request(request) => {false}
            Resource::Profile(profile) => {false}
            Resource::Review(review) => {false}
            Resource::Skill(skill) => {false}
            Resource::Role(role) => {false}
            Resource::WalletType(wallet_type) => {false}
            
            // NOSQL
            Resource::Notification(notification) => {false}
            Resource::WishList(wish_list) => {false}
            Resource::Conversation(conversation) => {false}
            Resource::Portfolio(portfolio) => {false}
            Resource::Message(message) => {false}
            
            _ => {false}
        };
        return Json(Response{authorized: is_authorized})
    }
    Json(Response { authorized: false })
}

// async fn check_route_authorization(Json(payload): Json<RouteAuthorizationParams>) -> Json<Response> {
//     let settings = Settings::new();
//     let postgres = PostgresClient::new(&settings.sql.connection_string).await.expect("Failed to connect to Postgres");
// 
//     let query: &str = "SELECT * FROM users JOIN roles ON users.role_id = roles.role_id WHERE users.user_id = $1";
//     let user: Option<User> = postgres.get_item_by_id(
//         &payload.user_id,
//         query
//     ).await.unwrap_or_else(|e| {
//         eprintln!("Database error: {:?}", e);
//         None
//     });
//     let all_routes: Vec<Route> = Route::get_routes().await;
//     let valid = all_routes.iter().any(|route| route.path == payload.route);
//     if !valid {
//         return Json(Response { authorized: false });
//     }
// 
//     let route: Option<&Route> = all_routes.iter().find(|route| route.path == payload.route);
//     if let Some(route) = route {
//         if !route.is_private {
//             return Json(Response { authorized: true });
//         }
//     }
// 
// 
//     Json(
//         Response { authorized: true }
//     )
// }

#[derive(Debug)]
pub enum Resource {
    // SQL
    User(User),
    Category(Category),
    SubCategory(SubCategory),
    Transaction(Transaction),
    Order(Order),
    Proposal(Proposal),
    Milestone(Milestone),
    Service(Service),
    Request(Request),
    Profile(Profile),
    Review(Review),
    Skill(Skill),
    Role(Role),
    WalletType(WalletType),
    // NoSQL
    Notification(Notification),
    WishList(WishList),
    Portfolio(Portfolio),
    Conversation(Conversation),
    Message(Message)
}

pub async fn fetch_resource_by_type(
    db: &DbClients,
    resource_type: &str,
    resource_id: &str,
) -> Option<Resource> {
    match resource_type {
        "sub_category" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <SubCategory as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::SubCategory)
        }
        "category" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <Category as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Category)
        }
        "transaction" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <Transaction as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Transaction)
        }
        "order" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <Order as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Order)
        }
        "proposal" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <Proposal as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Proposal)
        }
        "milestone" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <Milestone as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Milestone)
        }
        "service" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <Service as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Service)
        }
        "request" => {
            let id = resource_id.parse::<i32>().ok()?;
            <Request as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Request)
        }
        "profile" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <Profile as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Profile)
        }
        "review" => {
            let id: i32 = resource_id.parse::<i32>().ok()?;
            <Review as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::Review)
        }
        "user" => {
            let id = resource_id.parse::<i32>().ok()?;
            <User as FetchableResource>::fetch_by_id(&db, id).await.map(Resource::User)
        }
        
        // NO SQL
        "portfolio" => {
            Portfolio::fetch_by_id(&db, resource_id.to_string()).await.map(Resource::Portfolio)
        }
        "notification" => {
            Notification::fetch_by_id(&db, resource_id.to_string()).await.map(Resource::Notification)
        }
        "wishlist" => {
            WishList::fetch_by_id(&db, resource_id.to_string()).await.map(Resource::WishList)
        }
        "conversation" => {
            Conversation::fetch_by_id(&db, resource_id.to_string()).await.map(Resource::Conversation)
        }
        _ => None,
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/authorization", get(check_authorization))
        // .route("/authorization/route", get(check_route_authorization))
}