use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct HarryPotterCard {
    name: String,
    magic: u32,
    cunning: u32,
    courage: u32,
    wisdom: u32,
    temper: u32,
}

#[derive(Deserialize, Debug)]
pub struct DinosaurCard {
    name: String,
    eating_habits: String,
    height: f32,
    weight: f32,
    length: f32,
    killer_rating: u32,
    intelligence: u32,
    age: u32,
}

trait Card {
    fn show(&self);
}

impl Card for HarryPotterCard {
    fn show(&self) {
        println!("{:#?}", &self)
    }
}

impl Card for DinosaurCard {
    fn show(&self) {
        println!(
            "
---------------------------
{}
-----
eating_habits: {}
height: {}
weight: {}
length: {}
killer_rating: {}
intelligence: {}
age: {}
----------------------------
",
            self.name,
            self.eating_habits,
            self.height,
            self.weight,
            self.length,
            self.killer_rating,
            self.intelligence,
            self.age
        );
    }
}

pub struct Deck {
    cards: Vec<Box<dyn Card>>,
}

impl Deck {
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn new(file_path: &str) -> Deck {
        let data = fs::read_to_string(file_path).expect("Unable to read file");
        let mut deck = Deck { cards: Vec::new() };
        let mut reader = csv::Reader::from_reader(data.as_bytes());
        for row in reader.deserialize() {
            
            let card: DinosaurCard = match row {
                Ok(card) => card,
                Err(e) => panic!("couldnt create new deck {}", e),
            };
            deck.cards.push(Box::new(card))
        }

        deck
    }

    pub fn show(&self) {
        for card in self.cards.iter() {
            card.show();
        }
    }
}
