// mod player;
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    // cards: Vec<Card>,
}


impl Player {
    pub fn new(name: String) -> Self {
        Player {name}
    }
}