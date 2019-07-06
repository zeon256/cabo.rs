use crate::models::card::CardValue;
use crate::models::player::Player;
use crate::message::GameError;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum GameState {
    Init,
    Started,
    Ended,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub lobby_password: String,
    pub players: Vec<Player>,
    pub deck: Deck,
    pub curr_state: GameState,
}

impl Game {
    pub fn new(id: u32, lobby_password: &str) -> Self {
        Game {
            id,
            lobby_password: lobby_password.to_string(),
            players: Vec::with_capacity(4),
            deck: Deck::new(),
            curr_state: GameState::Init,
        }
    }

    pub fn player_join(&mut self, player: Player) {
        self.players.push(player)
    }

    /// Checks if lobby_password is filled
    /// Checks if there are more than 2 players
    /// Checks if curr_state is GameState::Init
    ///
    /// Once all checks are done, distribute cards to players
    /// evenly, eg. Top card goes to player 0, then next card goes to player 1 etc
    ///
    /// Then players are given 5s to look at their side cards
    ///
    /// Then generate a random number from 0 to max number of players
    /// Random number denotes the first player to start the draw
    ///
    /// Then game starts
    pub fn start(&mut self) -> Result<(), GameError>{
        if self.lobby_password.is_empty() {
            return Err(GameError::EmptyLobbyPassword)
        }

        if self.players.len() < 2 {
            return Err(GameError::NotEnoughPlayers)
        }

        self.curr_state = match self.curr_state {
            GameState::Init => GameState::Started,
            _ => return Err(GameError::WrongPrevGameState)
        };

        // distribute cards
         let idx_diff = self.players.len();

        for idx in 0..idx_diff {
            self.players[idx].cards.push(self.deck.cards.remove(idx));
            self.players[idx].cards.push(self.deck.cards.remove(idx + idx_diff));
            self.players[idx].cards.push(self.deck.cards.remove(idx + idx_diff + idx_diff));
        }

        println!("{:?}", self.players);
        Ok(())
    }

    /// Check if game has ended
    /// if yes, kick all players,
    /// and dont accept any new websocket messages
    pub fn has_ended() {
        unimplemented!()
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
