use crate::bitboard::{BitBoard, EMPTY};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BISHOP_MOVES: [BitBoard; 64] = init_bishop_moves();
}

fn init_bishop_moves() -> [BitBoard; 64] {
    let mut moves = [EMPTY; 64];
    for i in 0..64 {
        moves[i as usize] = all_bishop_moves(BitBoard::from_index(i));
    }
    moves
}

fn all_bishop_moves(bishop_bitboard: BitBoard) -> BitBoard {
    let mut moves = EMPTY;

    let mut up_right = bishop_bitboard;
    while up_right.get_rank() < 7 && up_right.get_file() < 7 {
        up_right = up_right.shift_up().shift_right();
        moves |= up_right;
    }

    let mut up_left = bishop_bitboard;
    while up_left.get_rank() < 7 && up_left.get_file() > 0 {
        up_left = up_left.shift_up().shift_left();
        moves |= up_left;
    }

    let mut down_right = bishop_bitboard;
    while down_right.get_rank() > 0 && down_right.get_file() < 7 {
        down_right = down_right.shift_down().shift_right();
        moves |= down_right;
    }

    let mut down_left = bishop_bitboard;
    while down_left.get_rank() > 0 && down_left.get_file() > 0 {
        down_left = down_left.shift_down().shift_left();
        moves |= down_left;
    }

    moves
}

pub fn generate_blocked_moves(bishop_bitboard: BitBoard, blockers: BitBoard) -> BitBoard {
    let mut moves = EMPTY;

    let mut up_right = bishop_bitboard;
    while up_right.get_rank() < 7 && up_right.get_file() < 7 {
        up_right = up_right.shift_up().shift_right();
        moves |= up_right;
        if up_right & blockers != EMPTY {
            break;
        }
    }

    let mut up_left = bishop_bitboard;
    while up_left.get_rank() < 7 && up_left.get_file() > 0 {
        up_left = up_left.shift_up().shift_left();
        moves |= up_left;
        if up_left & blockers != EMPTY {
            break;
        }
    }

    let mut down_right = bishop_bitboard;
    while down_right.get_rank() > 0 && down_right.get_file() < 7 {
        down_right = down_right.shift_down().shift_right();
        moves |= down_right;
        if down_right & blockers != EMPTY {
            break;
        }
    }

    let mut down_left = bishop_bitboard;
    while down_left.get_rank() > 0 && down_left.get_file() > 0 {
        down_left = down_left.shift_down().shift_left();
        moves |= down_left;
        if down_left & blockers != EMPTY {
            break;
        }
    }

    moves
}
