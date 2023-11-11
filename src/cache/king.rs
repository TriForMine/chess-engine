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
        moves |= king_bitboard.shift_up();
        if king_bitboard.get_file() < 7 {
            moves |= king_bitboard.shift_up().shift_right();
        }
        if king_bitboard.get_file() > 0 {
            moves |= king_bitboard.shift_up().shift_left();
        }
    }

    if king_bitboard.get_rank() > 0 {
        moves |= king_bitboard.shift_down();
        if king_bitboard.get_file() < 7 {
            moves |= king_bitboard.shift_down().shift_right();
        }
        if king_bitboard.get_file() > 0 {
            moves |= king_bitboard.shift_down().shift_left();
        }
    }

    if king_bitboard.get_file() < 7 {
        moves |= king_bitboard.shift_right();
    }

    if king_bitboard.get_file() > 0 {
        moves |= king_bitboard.shift_left();
    }

    moves
}
