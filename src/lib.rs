mod model;
mod game; 
mod player; 

use game::{Game, GameType};
use std::error::Error;

pub fn run() ->  Result<(), Box<dyn Error>> {

    let game = &mut game::Game::new(GameType::Dinosaur);

    for i in 0..10 {
        game.play()
    }

    // println!("player to move: {:#?}", game.players[game.player_to_move].name);
    Ok(())

}


