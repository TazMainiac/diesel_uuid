use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_user(conn: &mut PgConnection, name: &str) -> UuidUser {
    use crate::schema::uuid_users;

    let new_user = NewUser { name };

    diesel::insert_into(uuid_users::table)
        .values(&new_user)
        .returning(UuidUser::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}
