use crate::models::player::Player;
use crate::models::card::CardValue;

use rand::{thread_rng, Rng};

pub struct Game {
    pub lobby_password: String,
    pub players: Vec<Player>,
    pub deck: Deck
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<CardValue>
}

impl Deck {
    pub fn new() -> Deck {
        let mut d = Deck {
            cards: Vec::new()
        };

        for val in 1..=13 {
            for suit in 1..=4 {
                d.cards.push(CardValue::from_primitive(val, suit));
            }
        }



        d
    }
}