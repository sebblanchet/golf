use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hand {
    Left,
    Right,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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

impl fmt::Display for Shot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
