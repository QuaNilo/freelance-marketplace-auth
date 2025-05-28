use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::db::mongo::MongoClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct WishList {
    user_id: i32,
    lists: Vec<WishListData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WishListData {
    creation_date: DateTime<Utc>,
    description: String,
    lists: Vec<WishListItems>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WishListItems {
    services: Vec<i32>,
    requests: Vec<i32>
}

impl WishList {
    pub async fn get_wishlist(mongo: &MongoClient, wishlist_id: &str) -> Option<WishList> {
        let wishlist: Option<WishList> = mongo.get_by_field(
            "Wishlist",
            "_id",
            wishlist_id
        ).await.unwrap_or_else(| error | {
            eprintln!("Error getting wishlist: {}", error);
            None
        });
        wishlist
    }
}