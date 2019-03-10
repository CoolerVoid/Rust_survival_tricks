#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use models::NewUser;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok(); 
    let database_url = "database/test.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &SqliteConnection, name: &str, email: &str, password: &str) -> bool {
    use schema::users;

    let new_user = NewUser { name, email, password };

    let rows_inserted=diesel::insert_into(users::table) 
        .values(&new_user)
        .execute(conn);

    match rows_inserted {
	Err(_) => return false,
	Ok(_) => return true,
    }    
}


