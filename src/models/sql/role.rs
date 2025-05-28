use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone)]
pub enum RoleEnum {
    Admin,
    User,
    Both,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Role {
    role_id: i32,
    deleted: bool,
    role_name: String,
    role_description: String
}