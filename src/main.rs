#![feature(proc_macro_hygiene, decl_macro)]
mod models;
use self::models::*;
#[macro_use] extern crate rocket;


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
