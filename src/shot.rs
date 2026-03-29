use bevy::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hand {
    _Left = 1,
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
    _Push, // L
    _Slice,
    _Fade,
    Straight,
    _Draw,
    _Hook,
    _Pull, // R
}

impl Shot {
    // TODO
    fn _value(&self) -> Vec3 {
        match *self {
            Shot::_Push => Vec3::new(0., 0., 0.),
            Shot::_Slice => Vec3::new(0., 0., 0.),
            Shot::_Fade => Vec3::new(0., 0., 0.),
            Shot::Straight => Vec3::new(0., 0., 0.),
            Shot::_Draw => Vec3::new(0., 0., 0.),
            Shot::_Hook => Vec3::new(0., 0., 0.),
            Shot::_Pull => Vec3::new(0., 0., 0.),
        }
    }
}

impl fmt::Display for Shot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_lowercase();
        write!(f, "{}", s)
    }
}
