extern crate diesel;
extern crate UserManagerCrud;

use self::diesel::prelude::*;
use self::UserManagerCrud::*;
use std::env::args;

fn main() {
    use UserManagerCrud::schema::users::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(users.filter(name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting user");

    println!("Deleted {} user", num_deleted);
}
