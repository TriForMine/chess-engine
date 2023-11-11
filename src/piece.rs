use crate::coord::Coord;
use crate::evaluate::{
    BISHOP_SCORE, KING_SCORE, KNIGHT_SCORE, PAWN_SCORE, QUEEN_SCORE, ROOK_SCORE,
};

#[derive(PartialEq, Clone, Copy)]
pub enum PieceEnum {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceEnum {
    pub fn from_index(index: u8) -> PieceEnum {
        match index {
            0 => PieceEnum::Pawn,
            1 => PieceEnum::Knight,
            2 => PieceEnum::Bishop,
            3 => PieceEnum::Rook,
            4 => PieceEnum::Queen,
            5 => PieceEnum::King,
            _ => PieceEnum::Pawn,
        }
    }

    pub fn to_index(&self) -> u8 {
        match self {
            PieceEnum::Pawn => 0,
            PieceEnum::Knight => 1,
            PieceEnum::Bishop => 2,
            PieceEnum::Rook => 3,
            PieceEnum::Queen => 4,
            PieceEnum::King => 5,
        }
    }

    pub fn from_char(c: char) -> Option<PieceEnum> {
        match c {
            'P' => Some(PieceEnum::Pawn),
            'N' => Some(PieceEnum::Knight),
            'B' => Some(PieceEnum::Bishop),
            'R' => Some(PieceEnum::Rook),
            'Q' => Some(PieceEnum::Queen),
            'K' => Some(PieceEnum::King),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            PieceEnum::Pawn => 'P',
            PieceEnum::Knight => 'N',
            PieceEnum::Bishop => 'B',
            PieceEnum::Rook => 'R',
            PieceEnum::Queen => 'Q',
            PieceEnum::King => 'K',
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Piece {
    pub piece: PieceEnum,
    pub color: bool,
}

impl Piece {
    pub fn new(piece: PieceEnum, color: bool) -> Piece {
        Piece { piece, color }
    }

    pub fn from_index(index: u8, color: bool) -> Piece {
        Piece {
            piece: PieceEnum::from_index(index),
            color,
        }
    }

    pub fn from_char(c: char) -> Option<Piece> {
        let color = c.is_uppercase();
        let piece = PieceEnum::from_char(c.to_ascii_uppercase())?;

        Some(Piece::new(piece, color))
    }

    pub fn to_char(&self) -> char {
        if self.color {
            self.piece.to_char().to_ascii_uppercase()
        } else {
            self.piece.to_char().to_ascii_lowercase()
        }
    }

    pub fn to_index(&self) -> u8 {
        match self.piece {
            PieceEnum::Pawn => 0,
            PieceEnum::Knight => 1,
            PieceEnum::Bishop => 2,
            PieceEnum::Rook => 3,
            PieceEnum::Queen => 4,
            PieceEnum::King => 5,
        }
    }

    pub fn get_score(&self, coord: Coord, turn: bool) -> i16 {
        match self.piece {
            PieceEnum::Pawn => {
                PAWN_SCORE[if turn == self.color { 0 } else { 1 }][coord.to_index() as usize]
            }
            PieceEnum::Knight => {
                KNIGHT_SCORE[if turn == self.color { 0 } else { 1 }][coord.to_index() as usize]
            }
            PieceEnum::Bishop => {
                BISHOP_SCORE[if turn == self.color { 0 } else { 1 }][coord.to_index() as usize]
            }
            PieceEnum::Rook => {
                ROOK_SCORE[if turn == self.color { 0 } else { 1 }][coord.to_index() as usize]
            }
            PieceEnum::Queen => {
                QUEEN_SCORE[if turn == self.color { 0 } else { 1 }][coord.to_index() as usize]
            }
            PieceEnum::King => {
                KING_SCORE[if turn == self.color { 0 } else { 1 }][coord.to_index() as usize]
            }
        }
    }

    pub fn is_white(&self) -> bool {
        self.color
    }

    pub fn is_black(&self) -> bool {
        !self.color
    }

    pub fn is_pawn(&self) -> bool {
        self.piece == PieceEnum::Pawn
    }

    pub fn is_knight(&self) -> bool {
        self.piece == PieceEnum::Knight
    }

    pub fn is_bishop(&self) -> bool {
        self.piece == PieceEnum::Bishop
    }

    pub fn is_rook(&self) -> bool {
        self.piece == PieceEnum::Rook
    }

    pub fn is_queen(&self) -> bool {
        self.piece == PieceEnum::Queen
    }

    pub fn is_king(&self) -> bool {
        self.piece == PieceEnum::King
    }
}
