
pub trait Card {
    fn show(&self);
    // fn clone_dyn(&self) -> Self;
    fn cmp(&self, other:&Self, field: &str) -> bool;
}


impl Card for HarryPotterCard {
    // fn clone_dyn(&self) -> Self {
    //     Box::new(self.clone()) // Forward to the derive(Clone) impl
    // }
    fn show(&self) {
        println!("{:#?}", &self)
    }

    fn cmp(&self, other:Box<Self>, field: &str) -> bool {
        todo!()
    }
}

impl Card for DinosaurCard {
    fn cmp(&self, other: &DinosaurCard, field: &str) -> bool {
        match field {
            "height" => self.height > other.height,
            "weight" => self.weight > other.weight,
            _ => panic!("not a real field"),
        }
    }

    // fn clone_dyn(&self) -> Self {
    //     Box::new(self.clone()) // Forward to the derive(Clone) impl
    // }
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
