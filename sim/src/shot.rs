use bevy::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hand {
    Left = 1,
    Right = -1,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_lowercase();
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shot {
    Push, // L
    Slice,
    Fade,
    Straight,
    Draw,
    Hook,
    Pull, // R
}

impl Shot {
    // TODO
    fn _value(&self) -> Vec3 {
        match *self {
            Shot::Push => Vec3::new(0., 0., 0.),
            Shot::Slice => Vec3::new(0., 0., 0.),
            Shot::Fade => Vec3::new(0., 0., 0.),
            Shot::Straight => Vec3::new(0., 0., 0.),
            Shot::Draw => Vec3::new(0., 0., 0.),
            Shot::Hook => Vec3::new(0., 0., 0.),
            Shot::Pull => Vec3::new(0., 0., 0.),
        }
    }
}

impl fmt::Display for Shot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_lowercase();
        write!(f, "{}", s)
    }
}
