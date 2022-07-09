
use crate::model::{Player, Card, Deck};


pub struct Game {
    pub ended: bool,
    pub player_to_move: usize,
    pub players: Vec<Player>,
}

pub enum GameType {
    Dinosaur,
    HarryPotter,
}

impl Game{
    pub fn new(game_type: GameType) -> Self {
        let file_path = match game_type {
            GameType::Dinosaur =>"src/data/dinosaurs.csv",
            GameType::HarryPotter => "src/data/harry_potter.csv",
        };
        let mut deck = Deck::new(file_path);

        // shuffle deck
        deck.shuffle();
        // deck.show();

        let midway = deck.cards.len() / 2;

        
        let player1 = Player::new("player1".to_string(), deck.cards[..midway].to_vec());
        let player2 = Player::new("player2".to_string(), deck.cards[midway..].to_vec());

        Game {
            ended: false,
            player_to_move: 0,
            players: Vec::from([player1, player2]),
        }
    }

    fn next_move(&mut self) {
        self.player_to_move = (self.player_to_move + 1) % 2;
    }

    pub fn play(&mut self) {
        // let current_player = self.players[self.player_to_move].cards[0];
        // let other_player = &self.players[self.player_to_move + 1 % 2];

        
        println!("{}", self.player_to_move);
        println!("{}", self.players[self.player_to_move].name);
        let first_card = &self.players[self.player_to_move].cards.remove(0);
        println!("{:#?}", first_card.show());

        // self.players[self.player_to_move + 1 % 2].cards.push(self.players[self.player_to_move].cards[0]);
        self.next_move();

    }

    
}

