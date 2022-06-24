mod model;
mod game; 
mod player; 

use game::{Game, GameType};
use std::error::Error;

pub fn run() ->  Result<(), Box<dyn Error>> {

    let mut game = game::Game::new(GameType::Dinosaur);

    println!("player to move: {:#?}", game.player_to_move);
    Ok(())

}


