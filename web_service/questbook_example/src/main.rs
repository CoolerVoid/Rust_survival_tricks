#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

mod questbook;

use rocket::fairing::AdHoc;
use rocket::request::Form;
use rocket_contrib::json::Json;
use diesel::SqliteConnection;
use questbook::{Questbook, NewQuest, DeleteQuest, UpdateQuest};

embed_migrations!();

#[database("sqlite_database")]
pub struct Db(SqliteConnection);

#[post("/new_questbook", data = "<questbook_form>")]
fn insert(questbook_form: Form<NewQuest>, conn: Db) {
    let questbook = questbook_form.into_inner();
    Questbook::insert(questbook, &conn);
}


#[post("/delete_questbook", data = "<questbook_form>")]
fn remove(questbook_form: Form<DeleteQuest>, conn: Db) {
    let questbook = questbook_form.into_inner();
    Questbook::delete(questbook, &conn);
}

#[post("/update_questbook", data = "<questbook_form>")]
fn update(questbook_form: Form<UpdateQuest>, conn: Db) {
    let questbook = questbook_form.into_inner();
    Questbook::update(questbook, &conn);
}

#[get("/list")]
fn list(conn: Db) -> Json<Vec<Questbook>> {
    Json(Questbook::all(&conn))
}


fn main() {
    rocket::ignite()
        .attach(Db::fairing())
        .attach(AdHoc::on_attach("Database Migrations", |rocket| {
            let conn = Db::get_one(&rocket).expect("no database connection");
            match embedded_migrations::run(&*conn) {
                Ok(_) => Ok(rocket),
                Err(err) => {
                    error!("Failed to run database migrations: {:?}", err);
                    Err(rocket)
                },
            }
        }))
        .mount("/", routes![list, insert, remove, update])
        .launch();
}
