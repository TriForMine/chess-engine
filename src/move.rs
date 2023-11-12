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

    pub fn is_capture(&self) -> bool {
        self.capture
    }

    pub fn from_str(s: &str) -> Move {
        let from = Coord::from_str(&s[0..2]).unwrap();
        let to = Coord::from_str(&s[2..4]).unwrap();

        let mut capture = false;
        let mut promotion = false;

        if s.len() == 5 {
            if s.chars().nth(4).unwrap() == 'q' {
                promotion = true;
            } else {
                capture = true;
            }
        } else if s.len() == 6 {
            capture = true;
            promotion = true;
        }

        Move::new(from, to, capture, promotion)
    }

    pub fn to_str(&self) -> String {
        let mut s = format!("{}{}", self.from.to_str(), self.to.to_str());

        if self.promotion {
            s.push('q');
        }

        s
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
