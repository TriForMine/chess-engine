use crate::bitboard::{BitBoard, EMPTY};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PAWN_MOVES: [[BitBoard; 64]; 2] = init_pawn_moves();
    pub static ref PAWN_ATTACKS: [[BitBoard; 64]; 2] = init_pawn_attacks();
}

fn init_pawn_moves() -> [[BitBoard; 64]; 2] {
    let mut moves = [[EMPTY; 64]; 2];
    for i in 0..64 {
        moves[0][i as usize] = all_pawn_moves(BitBoard::from_index(i), false);
        moves[1][i as usize] = all_pawn_moves(BitBoard::from_index(i), true);
    }
    moves
}

fn init_pawn_attacks() -> [[BitBoard; 64]; 2] {
    let mut moves = [[EMPTY; 64]; 2];
    for i in 0..64 {
        moves[0][i as usize] = all_pawn_attacks(BitBoard::from_index(i), false);
        moves[1][i as usize] = all_pawn_attacks(BitBoard::from_index(i), true);
    }
    moves
}

fn all_pawn_moves(pawn_bitboard: BitBoard, color: bool) -> BitBoard {
    let mut moves = EMPTY;

    if color {
        moves |= pawn_bitboard.shift_up();
        if pawn_bitboard.get_rank() == 1 {
            moves |= pawn_bitboard.shift_up().shift_up();
        }
    } else {
        moves |= pawn_bitboard.shift_down();
        if pawn_bitboard.get_rank() == 6 {
            moves |= pawn_bitboard.shift_down().shift_down();
        }
    }

    moves
}

fn all_pawn_attacks(pawn_bitboard: BitBoard, color: bool) -> BitBoard {
    let mut moves = EMPTY;

    if color {
        moves |= pawn_bitboard.shift_up().shift_right();
        moves |= pawn_bitboard.shift_up().shift_left();
    } else {
        moves |= pawn_bitboard.shift_down().shift_right();
        moves |= pawn_bitboard.shift_down().shift_left();
    }

    moves
}
