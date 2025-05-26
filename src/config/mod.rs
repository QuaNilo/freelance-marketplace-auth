use std::env;

#[derive(Debug)]
pub struct Mongo {
    pub connection_string: String,
    pub database_name: String,
}

impl Mongo {
    fn from_env() -> Self {
        Mongo {
            connection_string: env::var("MONGO_CONNECTION_STRING").unwrap_or_default(),
            database_name: env::var("MONGO_DATABASE_NAME").unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct Sql {
    pub connection_string: String,
}

impl Sql {
    fn from_env() -> Self {
        Sql {
            connection_string: env::var("SQL_CONNECTION_STRING").unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    pub mongo: Mongo,
    pub sql: Sql,
}


impl Settings {
    pub fn new() -> Self {
        Settings {
            mongo: Mongo::from_env(),
            sql: Sql::from_env(),
        }
    }
}