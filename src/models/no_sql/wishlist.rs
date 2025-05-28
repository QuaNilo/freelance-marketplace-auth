use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::traits::fetchable_resource::{DbClients, FetchableResource};

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

#[async_trait]
impl FetchableResource for WishList {
    type IdType = String;
    async fn fetch_by_id(db: &DbClients, wishlist_id: Self::IdType) -> Option<WishList> {
        let wishlist: Option<WishList> = db.mongo.get_by_field(
            "Wishlist",
            "_id",
            &wishlist_id
        ).await.unwrap_or_else(| error | {
            eprintln!("Error getting wishlist: {}", error);
            None
        });
        wishlist
    }
}