use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Club {
    pub name: String,
    pub loft: f32,    // loft of the club
    pub speed: f32,   // speed of the club when swung
    pub spin: f32,    // backspin rad/s
    pub weight: f32,  // weight of the club in kg
    pub inertia: f32, // inertia of club
}

impl Club {
    pub fn new(n: &str, loft: f32, speed: f32, spin: f32) -> Self {
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
        }
    }
}

impl Default for Club {
    fn default() -> Self {
        Self {
            name: "1w".to_string(),
            loft: 10.,
            speed: 50.,
            spin: 200.,
            weight: 0.2,
            inertia: 9.145e-6,
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
            Club::new("1w", 10., 50., 200.),
            Club::new("5i", 25., 45., 300.),
            Club::new("7i", 35., 40., 400.),
            Club::new("pw", 45., 35., 500.),
        ];
        Self { clubs }
    }
}
