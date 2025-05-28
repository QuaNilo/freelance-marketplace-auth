
pub async fn is_nosql_resource(resource: &str) -> bool {
    matches!(
        resource.to_lowercase().as_str(),
        "wishlist" | "notification" | "conversation" | "message" | "portfolio"
    )
}

pub async fn is_sql_resource(resource: &str) -> bool {
    matches!(
        resource.to_lowercase().as_str(),
        "user" | "wallettype" | "skill" | "role" | "profile" |
        "request" | "requeststatus" | "service" | "servicestatus" |
        "milestone" | "milestonestatus" | "proposal" | "proposalstatus" |
        "order" | "orderstatus" | "transaction" | "category" |
        "subcategory" | "review"
    )
}