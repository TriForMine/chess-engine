use crate::bitboard::{BitBoard, EMPTY};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KING_MOVES: [BitBoard; 64] = init_king_moves();
}

fn init_king_moves() -> [BitBoard; 64] {
    let mut moves = [EMPTY; 64];
    for i in 0..64 {
        moves[i as usize] = all_king_moves(BitBoard::from_index(i));
    }
    moves
}

fn all_king_moves(king_bitboard: BitBoard) -> BitBoard {
    let mut moves = EMPTY;

    if king_bitboard.get_rank() < 7 {
        let mut up = king_bitboard;
        up.shift_up();
        moves |= up;
        if king_bitboard.get_file() < 7 {
            let mut up_right = up;
            up_right.shift_right();
            moves |= up_right;
        }
        if king_bitboard.get_file() > 0 {
            let mut up_left = up;
            up_left.shift_left();
            moves |= up_left;
        }
    }

    if king_bitboard.get_rank() > 0 {
        let mut down = king_bitboard;
        down.shift_down();
        moves |= down;
        if king_bitboard.get_file() < 7 {
            let mut down_right = down;
            down_right.shift_right();
            moves |= down_right;
        }
        if king_bitboard.get_file() > 0 {
            let mut down_left = down;
            down_left.shift_left();
            moves |= down_left;
        }
    }

    if king_bitboard.get_file() < 7 {
        let mut right = king_bitboard;
        right.shift_right();
        moves |= right;
    }

    if king_bitboard.get_file() > 0 {
        let mut left = king_bitboard;
        left.shift_left();
        moves |= left;
    }

    moves
}
