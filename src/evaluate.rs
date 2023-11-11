use crate::board::Board;
use crate::coord::Coord;

pub const PAWN_SCORE: [[i16; 64]; 2] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, // White
        50, 50, 50, 50, 50, 50, 50, 50, // White
        10, 10, 20, 30, 30, 20, 10, 10, // White
        5, 5, 10, 25, 25, 10, 5, 5, // White
        0, 0, 0, 20, 20, 0, 0, 0, // White
        5, -5, -10, 0, 0, -10, -5, 5, // White
        5, 10, 10, -20, -20, 10, 10, 5, // White
        0, 0, 0, 0, 0, 0, 0, 0, // White
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, // Black
        5, 10, 10, -20, -20, 10, 10, 5, // Black
        5, -5, -10, 0, 0, -10, -5, 5, // Black
        0, 0, 0, 20, 20, 0, 0, 0, // Black
        5, 5, 10, 25, 25, 10, 5, 5, // Black
        10, 10, 20, 30, 30, 20, 10, 10, // Black
        50, 50, 50, 50, 50, 50, 50, 50, // Black
        0, 0, 0, 0, 0, 0, 0, 0, // Black
    ],
];

pub const KNIGHT_SCORE: [[i16; 64]; 2] = [
    [
        -50, -40, -30, -30, -30, -30, -40, -50, // White
        -40, -20, 0, 0, 0, 0, -20, -40, // White
        -30, 0, 10, 15, 15, 10, 0, -30, // White
        -30, 5, 15, 20, 20, 15, 5, -30, // White
        -30, 0, 15, 20, 20, 15, 0, -30, // White
        -30, 5, 10, 15, 15, 10, 5, -30, // White
        -40, -20, 0, 5, 5, 0, -20, -40, // White
        -50, -40, -30, -30, -30, -30, -40, -50, // White
    ],
    [
        -50, -40, -30, -30, -30, -30, -40, -50, // Black
        -40, -20, 0, 5, 5, 0, -20, -40, // Black
        -30, 5, 10, 15, 15, 10, 5, -30, // Black
        -30, 0, 15, 20, 20, 15, 0, -30, // Black
        -30, 5, 15, 20, 20, 15, 5, -30, // Black
        -30, 0, 10, 15, 15, 10, 0, -30, // Black
        -40, -20, 0, 0, 0, 0, -20, -40, // Black
        -50, -40, -30, -30, -30, -30, -40, -50, // Black
    ],
];

pub const BISHOP_SCORE: [[i16; 64]; 2] = [
    [
        -20, -10, -10, -10, -10, -10, -10, -20, // White
        -10, 0, 0, 0, 0, 0, 0, -10, // White
        -10, 0, 5, 10, 10, 5, 0, -10, // White
        -10, 5, 5, 10, 10, 5, 5, -10, // White
        -10, 0, 10, 10, 10, 10, 0, -10, // White
        -10, 10, 10, 10, 10, 10, 10, -10, // White
        -10, 5, 0, 0, 0, 0, 5, -10, // White
        -20, -10, -10, -10, -10, -10, -10, -20, // White
    ],
    [
        -20, -10, -10, -10, -10, -10, -10, -20, // Black
        -10, 5, 0, 0, 0, 0, 5, -10, // Black
        -10, 10, 10, 10, 10, 10, 10, -10, // Black
        -10, 0, 10, 10, 10, 10, 0, -10, // Black
        -10, 5, 5, 10, 10, 5, 5, -10, // Black
        -10, 0, 5, 10, 10, 5, 0, -10, // Black
        -10, 0, 0, 0, 0, 0, 0, -10, // Black
        -20, -10, -10, -10, -10, -10, -10, -20, // Black
    ],
];

pub const ROOK_SCORE: [[i16; 64]; 2] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, // White
        5, 10, 10, 10, 10, 10, 10, 5, // White
        -5, 0, 0, 0, 0, 0, 0, -5, // White
        -5, 0, 0, 0, 0, 0, 0, -5, // White
        -5, 0, 0, 0, 0, 0, 0, -5, // White
        -5, 0, 0, 0, 0, 0, 0, -5, // White
        -5, 0, 0, 0, 0, 0, 0, -5, // White
        0, 0, 0, 5, 5, 0, 0, 0, // White
    ],
    [
        0, 0, 0, 5, 5, 0, 0, 0, // Black
        -5, 0, 0, 0, 0, 0, 0, -5, // Black
        -5, 0, 0, 0, 0, 0, 0, -5, // Black
        -5, 0, 0, 0, 0, 0, 0, -5, // Black
        -5, 0, 0, 0, 0, 0, 0, -5, // Black
        -5, 0, 0, 0, 0, 0, 0, -5, // Black
        5, 10, 10, 10, 10, 10, 10, 5, // Black
        0, 0, 0, 0, 0, 0, 0, 0, // Black
    ],
];

pub const QUEEN_SCORE: [[i16; 64]; 2] = [
    [
        -20, -10, -10, -5, -5, -10, -10, -20, // White
        -10, 0, 0, 0, 0, 0, 0, -10, // White
        -10, 0, 5, 5, 5, 5, 0, -10, // White
        -5, 0, 5, 5, 5, 5, 0, -5, // White
        0, 0, 5, 5, 5, 5, 0, -5, // White
        -10, 5, 5, 5, 5, 5, 0, -10, // White
        -10, 0, 5, 0, 0, 0, 0, -10, // White
        -20, -10, -10, -5, -5, -10, -10, -20, // White
    ],
    [
        -20, -10, -10, -5, -5, -10, -10, -20, // Black
        -10, 0, 5, 0, 0, 0, 0, -10, // Black
        -10, 5, 5, 5, 5, 5, 0, -10, // Black
        0, 0, 5, 5, 5, 5, 0, -5, // Black
        -5, 0, 5, 5, 5, 5, 0, -5, // Black
        -10, 0, 5, 5, 5, 5, 0, -10, // Black
        -10, 0, 0, 0, 0, 0, 0, -10, // Black
        -20, -10, -10, -5, -5, -10, -10, -20, // Black
    ],
];

pub const KING_SCORE: [[i16; 64]; 2] = [
    [
        20, 30, 10, 0, 0, 10, 30, 20, // White
        20, 20, 0, 0, 0, 0, 20, 20, // White
        -10, -20, -20, -20, -20, -20, -20, -10, // White
        -20, -30, -30, -40, -40, -30, -30, -20, // White
        -30, -40, -40, -50, -50, -40, -40, -30, // White
        -30, -40, -40, -50, -50, -40, -40, -30, // White
        -30, -40, -40, -50, -50, -40, -40, -30, // White
        -30, -40, -40, -50, -50, -40, -40, -30, // White
    ],
    [
        -30, -40, -40, -50, -50, -40, -40, -30, // Black
        -30, -40, -40, -50, -50, -40, -40, -30, // Black
        -30, -40, -40, -50, -50, -40, -40, -30, // Black
        -30, -40, -40, -50, -50, -40, -40, -30, // Black
        -20, -30, -30, -40, -40, -30, -30, -20, // Black
        -10, -20, -20, -20, -20, -20, -20, -10, // Black
        20, 20, 0, 0, 0, 0, 20, 20, // Black
        20, 30, 10, 0, 0, 10, 30, 20, // Black
    ],
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#move::Move;

    #[test]
    fn test_score() {
        let board = Board::new_game();

        assert_eq!(board.evaluate(), 0);
    }
}
