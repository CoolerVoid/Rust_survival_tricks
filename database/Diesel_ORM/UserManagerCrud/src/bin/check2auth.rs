extern crate diesel;
extern crate UserManagerCrud;

use self::diesel::prelude::*;
use self::UserManagerCrud::models::*;
use self::UserManagerCrud::*;
use std::io::stdin;
mod lambda_crypt;


fn main() {
    use UserManagerCrud::schema::users::dsl::*;

    println!("Put you email here:");
    let mut emailin = String::new();
    stdin().read_line(&mut emailin).unwrap();
    let emailin = &emailin[..(emailin.len() - 1)]; // Drop the newline character

    println!("Put you password here:");
    let mut passwordin = String::new();
    stdin().read_line(&mut passwordin).unwrap();
    let passwordin = &passwordin[..(passwordin.len() - 1)]; // Drop the newline character
    let cryptpass = lambda_crypt::get_hash(passwordin);
  //  use schema::users;
    let connection = establish_connection();

    let result = users.filter(email.eq(&emailin)).filter(password.eq(&cryptpass))
	.load::<User>(&connection)
	.expect("Check user query failed to run");
    
    println!("Result : {:?}",result.len());
}
