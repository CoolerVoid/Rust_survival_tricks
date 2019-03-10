extern crate diesel;
extern crate UserManagerCrud;

use self::UserManagerCrud::*;
use std::io::stdin;
mod lambda_crypt;

fn main() {
    let connection = establish_connection();

    println!("Put you nick here:");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character

    println!("Put you email here:");
    let mut email = String::new();
    stdin().read_line(&mut email).unwrap();
    let email = &email[..(email.len() - 1)]; // Drop the newline character

    println!("Put you password here:");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = &password[..(password.len() - 1)]; // Drop the newline character
    let cryptpass = lambda_crypt::get_hash(password);
    if create_user(&connection, name, email, &cryptpass) == true {
     	println!("Saved User! {}", name);
    } else {
	println!("Error in save user: {}",name);
    } 
}

