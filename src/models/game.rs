use crate::models::card::CardValue;
use crate::models::player::Player;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub enum GameState {
    Init,
    Started,
    Ended,
}

#[derive(Debug)]
pub struct Game {
    pub lobby_password: String,
    pub players: Vec<Player>,
    pub deck: Deck,
    pub curr_state: GameState,
}

impl Game {
    pub fn new(lobby_password: &str, players: Vec<Player>) -> Self {
        Game {
            lobby_password: lobby_password.to_string(),
            players,
            deck: Deck::new(),
            curr_state: GameState::Init,
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<CardValue>,
}

impl Deck {
    pub fn new() -> Self {
        let mut d = Deck { cards: Vec::new() };
        for val in 1..=13 {
            for suit in 1..=4 {
                d.cards.push(CardValue::from_primitive(val, suit));
            }
        }

        d
    }

    pub fn shuffle(mut self) -> Self {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
        self
    }
}
