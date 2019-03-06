extern crate diesel;
extern crate UserManagerCrud;

use self::diesel::prelude::*;
use self::UserManagerCrud::models::*;
use self::UserManagerCrud::*;

fn main() {
    use UserManagerCrud::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(10)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("Name: {}", user.name);
        println!("Email: {}", user.email);
        println!("Password: {}", user.password);
    }
}
