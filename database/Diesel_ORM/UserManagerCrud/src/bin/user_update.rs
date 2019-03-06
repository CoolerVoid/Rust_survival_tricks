extern crate diesel;
extern crate UserManagerCrud;

use self::diesel::prelude::*;
//use self::UserManagerCrud::models::*;
use self::UserManagerCrud::*;
use std::io::stdin;
mod lambda_crypt;



fn main() {
    let connection = establish_connection();

    use UserManagerCrud::schema::users::dsl::*;

    println!("Put you nick here:");
    let mut namein = String::new();
    stdin().read_line(&mut namein).unwrap();
    let namein = &namein[..(namein.len() - 1)]; // Drop the newline character

    println!("Put you email here:");
    let mut emailin = String::new();
    stdin().read_line(&mut emailin).unwrap();
    let emailin = &emailin[..(emailin.len() - 1)]; // Drop the newline character

    println!("Put you password here:");
    let mut passwordin = String::new();
    stdin().read_line(&mut passwordin).unwrap();
    let passwordin = &passwordin[..(passwordin.len() - 1)]; // Drop the newline character
// using sha512
    let cryptpass = lambda_crypt::get_hash(passwordin);
    let _ = diesel::update(users.filter(name.eq(namein)))
    .set( (email.eq(emailin), password.eq(cryptpass)) )
    .execute(&connection)
    .unwrap_or_else(|_| panic!("Unable to find name {} to edit", namein));
    println!("\nUpdate user email and password" );
}

