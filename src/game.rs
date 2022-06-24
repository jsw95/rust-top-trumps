
use crate::player::Player;

use crate::model;

pub struct Game {
    pub player_to_move: Player,
    pub players: Vec<Player>,
}

pub enum GameType {
    Dinosaur,
    HarryPotter,
}

impl Game {
    pub fn new(game_type: GameType) -> Self {
        let file_path = match game_type {
            GameType::Dinosaur =>"src/data/dinosaurs.csv",
            GameType::HarryPotter => "src/data/harry_potter.csv",
        };
        let mut deck = model::Deck::new(file_path);
        deck.show();

        // shuffle deck
        deck.shuffle();
        deck.show();
        let player1 = Player::new("player1".to_string());
        let player2 = Player::new("player2".to_string());
        Game {
            player_to_move: player1.clone(),
            players: Vec::from([player1.clone(), player2.clone()]),
        }
    }
}
