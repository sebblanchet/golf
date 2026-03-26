use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Club {
    pub name: String,
    pub loft: f32,    // loft of the club
    pub speed: f32,   // speed of the club when swung
    pub spin: f32,    // backspin rad/s
    pub weight: f32,  // weight of the club in kg
    pub inertia: f32, // inertia of club
    pub smash: f32,   // smash factor
}

impl Club {
    pub fn new(n: &str, loft: f32, speed: f32, spin: f32, smash: f32) -> Self {
        let weight = 0.2;
        let inertia = 9.145e-6;
        let name = n.to_string();

        Self {
            name,
            loft,
            speed,
            spin,
            weight,
            inertia,
            smash,
        }
    }
}

impl Default for Club {
    fn default() -> Self {
        Self {
            name: "Driver".to_string(),
            loft: 10.,
            speed: 42.,
            spin: 280.,
            weight: 0.2,
            inertia: 9.145e-6,
            smash: 1.5,
        }
    }
}

#[derive(Debug, Resource)]
pub struct Bag {
    pub clubs: Vec<Club>,
}

impl Bag {
    pub fn _new() -> Self {
        Bag { clubs: Vec::new() }
    }

    pub fn _get(&self, name: String) -> Club {
        for club in &self.clubs {
            if name == club.name {
                return club.clone();
            }
        }
        Club::default()
    }

    pub fn _list(&self) -> Vec<String> {
        let mut v: Vec<String> = self.clubs.clone().into_iter().map(|k| k.name).collect();
        v.sort();
        v
    }

    pub fn _insert(&mut self, club: Club) {
        self.clubs.push(club);
    }
}

impl Default for Bag {
    fn default() -> Self {
        let clubs = vec![
            Club::default(),
            Club::new("3-wood", 14., 39., 330., 1.5),
            Club::new("5-wood", 18., 38., 360., 1.5),
            Club::new("3-iron", 21., 37., 410., 1.5),
            Club::new("4-iron", 24., 36., 440., 1.5),
            Club::new("5-iron", 27., 35., 500., 1.5),
            Club::new("6-iron", 30., 34., 540., 1.5),
            Club::new("7-iron", 33., 33., 590., 1.5),
            Club::new("8-iron", 37., 32., 640., 1.5),
            Club::new("9-iron", 42., 30., 700., 1.5),
            Club::new("PW", 46., 29., 760., 1.5),
        ];
        Self { clubs }
    }
}
