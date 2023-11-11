use crate::bitboard::BitBoard;
use crate::coord::Coord;
use crate::piece::Piece;
use std::fmt::{Debug, Display};

#[derive(PartialEq, Clone, Copy)]
pub struct Move {
    pub from: Coord,
    pub to: Coord,
    pub capture: bool,
    pub promotion: bool,
}

impl Move {
    pub fn null() -> Move {
        Move {
            from: Coord::new(0, 0),
            to: Coord::new(0, 0),
            capture: false,
            promotion: false,
        }
    }

    pub fn new(from: Coord, to: Coord, capture: bool, promotion: bool) -> Move {
        Move {
            from,
            to,
            capture,
            promotion,
        }
    }

    pub fn from_str(s: &str) -> Option<Move> {
        if s.len() != 4 && s.len() != 5 {
            return None;
        }

        let from = Coord::from_str(&s[0..2])?;
        let to = Coord::from_str(&s[2..4])?;
        let piece = Piece::from_char(s.chars().nth(4)?)?;

        Some(Move::new(from, to, false, false))
    }

    pub fn to_str(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.from.to_str());
        s.push_str(&self.to.to_str());
        s
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&self.from.to_str());
        s.push_str(&self.to.to_str());
        write!(f, "{}", s)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&self.from.to_str());
        s.push_str(&self.to.to_str());
        write!(f, "{}", s)
    }
}
