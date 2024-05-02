use crate::schema::users;
use diesel::{pg::PgConnection, prelude::*, Queryable};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Debug, Clone, Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}
