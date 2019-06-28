#![feature(proc_macro_hygiene, decl_macro)]
mod models;
use self::models::*;
use crate::models::game::Deck;

#[macro_use] extern crate num_derive;
extern crate num_traits;

extern crate rand;

fn main() {

    let mut d = Deck::new();
    println!("{:?}", d);
}
