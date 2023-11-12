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
        let mut up = pawn_bitboard;
        up.shift_up();
        moves |= up;
        if pawn_bitboard.get_rank() == 1 {
            let mut up_up = up;
            up_up.shift_up();
            moves |= up_up;
        }
    } else {
        let mut down = pawn_bitboard;
        down.shift_down();
        moves |= down;
        if pawn_bitboard.get_rank() == 6 {
            let mut down_down = down;
            down_down.shift_down();
            moves |= down_down;
        }
    }

    moves
}

fn all_pawn_attacks(pawn_bitboard: BitBoard, color: bool) -> BitBoard {
    let mut moves = EMPTY;

    if color {
        if pawn_bitboard.get_file() < 7 {
            let mut up_right = pawn_bitboard;
            up_right.shift_up();
            up_right.shift_right();
            moves |= up_right;
        }
        if pawn_bitboard.get_file() > 0 {
            let mut up_left = pawn_bitboard;
            up_left.shift_up();
            up_left.shift_left();
            moves |= up_left;
        }
    } else {
        if pawn_bitboard.get_file() < 7 {
            let mut down_right = pawn_bitboard;
            down_right.shift_down();
            down_right.shift_right();
            moves |= down_right;
        }
        if pawn_bitboard.get_file() > 0 {
            let mut down_left = pawn_bitboard;
            down_left.shift_down();
            down_left.shift_left();
            moves |= down_left;
        }
    }

    moves
}
