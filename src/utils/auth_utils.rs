const SQL_RESOURCES: &[&str] = &[
    "user","wallet_type", "skill", "role", "profile",
        "request", "request_status", "service", "service_status",
        "milestone", "milestone_status", "proposal", "proposal_status",
        "order", "order_status", "transaction", "category",
        "subcategory", "review"
];

const NOSQL_RESOURCES: &[&str] = &[
    "wishlist", "notification", "conversation", "message", "portfolio"
];

pub async fn is_sql_resource(resource_type: &str) -> bool {
    SQL_RESOURCES.contains(&resource_type)
}

pub async fn is_nosql_resource(resource_type: &str) -> bool {
    NOSQL_RESOURCES.contains(&resource_type)
}