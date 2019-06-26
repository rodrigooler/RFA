#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

use db;
mod db;

#[get("/code/<id>")]
fn hello(id: u128) -> Json<Brazil> {
    let db: Database
}

// #[get("/code/<id>")]
// fn hello(id: u128) -> Json<Brazil> {
//     format!("Hello, {} year old named {}!", age, name)
// }

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}