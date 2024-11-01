use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Club {
    pub loft: f32,   // loft of the club
    pub speed: f32,  // Speed of the club when swung
    pub spin: f32,   // backspin rad/s
    pub weight: f32, // Weight of the club in kg
    pub inertia: f32,
}

impl Club {
    pub fn new(loft: f32, speed: f32, spin: f32) -> Self {
        // TODO
        let weight = 0.2;
        let inertia = 0.1;

        Self {
            loft,
            speed,
            spin,
            weight,
            inertia,
        }
    }
}

impl Default for Club {
    fn default() -> Self {
        Self {
            loft: 10.,
            speed: 50.,
            spin: 200.,
            weight: 1.,
            inertia: 1.,
        }
    }
}

#[derive(Debug, Resource)]
pub struct Bag {
    pub clubs: HashMap<String, Club>,
}

impl Bag {
    pub fn new() -> Self {
        Bag {
            clubs: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Club {
        let o = self.clubs.get(&name);

        if let Some(club) = o {
            club.clone()
        } else {
            Club::default()
        }
    }

    pub fn list(&self) -> Vec<String> {
        let mut v: Vec<String> = self.clubs.keys().map(|k| k.to_string()).collect();
        v.sort();
        v
    }

    pub fn insert(&mut self, name: String, club: Club) {
        self.clubs.insert(name, club);
    }

    pub fn total_weight(&self) -> f32 {
        self.clubs.values().map(|club| club.weight).sum()
    }
}

impl Default for Bag {
    fn default() -> Self {
        // default
        let w1 = Club::new(10., 50., 200.);
        let i5 = Club::new(25., 45., 300.);
        let i7 = Club::new(35., 40., 400.);
        let pw = Club::new(45., 35., 500.);

        // add clubs to the bag with unique names
        let mut clubs = HashMap::new();
        clubs.insert("1w".to_string(), w1);
        clubs.insert("5i".to_string(), i5);
        clubs.insert("7i".to_string(), i7);
        clubs.insert("pw".to_string(), pw);
        Self { clubs }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        // create a new golf bag
        let mut bag = Bag::new();

        // create some clubs
        let w1 = Club::new(10., 50., 100.);
        let i5 = Club::new(25., 45., 100.);
        let i7 = Club::new(35., 40., 100.);
        let pw = Club::new(45., 35., 100.);

        // add clubs to the bag with unique names
        bag.insert("1w".to_string(), w1);
        bag.insert("5i".to_string(), i5);
        bag.insert("7i".to_string(), i7);
        bag.insert("pw".to_string(), pw);

        // print the golf bag and total weight
        dbg!(bag);
    }
}
