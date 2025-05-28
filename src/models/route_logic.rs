use crate::models::sql::role::{RoleEnum};

#[derive(Debug, Clone)]
pub struct Route {
    pub path: &'static str,
    pub is_private: bool,
    pub allowed_roles: Option<RoleEnum>
}

impl Route {
    pub async fn get_routes() -> Vec<Route> {
        let routes = vec![
            Route { path: "/login", is_private: false, allowed_roles: None },
            Route { path: "/signup", is_private: false, allowed_roles: None },
            Route { path: "/jobs", is_private: false, allowed_roles: None },
            Route { path: "/jobs/:id", is_private: false, allowed_roles: None },
            Route { path: "profile", is_private: true, allowed_roles: Some(RoleEnum::Both) },
        ];
        routes
    }
    
    pub async fn get_public_routes() -> Vec<Route> {
        let routes = Self::get_routes().await;
        let public_routes: Vec<Route> = routes.iter().filter(|route| !route.is_private).cloned().collect();
        public_routes
    }
    
    
}