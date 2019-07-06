mod models;
use self::models::*;
use crate::models::game::{Deck, Game};
use crate::models::player::Player;

#[macro_use]
extern crate num_derive;
extern crate num_traits;

extern crate rand;

fn main() {
    let mut g = Game::new(5, "hello_world");
    g.player_join(Player::new(0, "jeff0"));
    g.player_join(Player::new(1, "jeff1"));
    g.player_join(Player::new(2, "jeff2"));
    g.player_join(Player::new(3, "jeff3"));

    let res = g.start();
    match res {
        Ok(r) => (),
        Err(e) => println!("{:?}", e),
    }
}
