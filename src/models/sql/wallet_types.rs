use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct WalletType {
    wallet_type_id: i32,
    deleted: bool,
    wallet_type_name: String
}

