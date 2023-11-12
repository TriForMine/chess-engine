use crate::bitboard::{BitBoard, EMPTY};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KNIGHT_MOVES: [BitBoard; 64] = init_knight_moves();
}

fn init_knight_moves() -> [BitBoard; 64] {
    let mut moves = [EMPTY; 64];
    for i in 0..64 {
        moves[i as usize] = all_knight_moves(BitBoard::from_index(i));
    }
    moves
}

fn all_knight_moves(knight_bitboard: BitBoard) -> BitBoard {
    let mut moves = EMPTY;

    if knight_bitboard.get_rank() < 6 && knight_bitboard.get_file() < 7 {
        let mut move1 = knight_bitboard;
        move1.shift_up();
        move1.shift_up();
        move1.shift_right();
        moves |= move1;
    }

    if knight_bitboard.get_rank() < 6 && knight_bitboard.get_file() > 0 {
        let mut move2 = knight_bitboard;
        move2.shift_up();
        move2.shift_up();
        move2.shift_left();
        moves |= move2;
    }

    if knight_bitboard.get_rank() > 1 && knight_bitboard.get_file() < 7 {
        let mut move3 = knight_bitboard;
        move3.shift_down();
        move3.shift_down();
        move3.shift_right();
        moves |= move3;
    }

    if knight_bitboard.get_rank() > 1 && knight_bitboard.get_file() > 0 {
        let mut move4 = knight_bitboard;
        move4.shift_down();
        move4.shift_down();
        move4.shift_left();
        moves |= move4;
    }

    if knight_bitboard.get_rank() < 7 && knight_bitboard.get_file() < 6 {
        let mut move5 = knight_bitboard;
        move5.shift_up();
        move5.shift_right();
        move5.shift_right();
        moves |= move5;
    }

    if knight_bitboard.get_rank() < 7 && knight_bitboard.get_file() > 1 {
        let mut move6 = knight_bitboard;
        move6.shift_up();
        move6.shift_left();
        move6.shift_left();
        moves |= move6;
    }

    if knight_bitboard.get_rank() > 0 && knight_bitboard.get_file() < 6 {
        let mut move7 = knight_bitboard;
        move7.shift_down();
        move7.shift_right();
        move7.shift_right();
        moves |= move7;
    }

    if knight_bitboard.get_rank() > 0 && knight_bitboard.get_file() > 1 {
        let mut move8 = knight_bitboard;
        move8.shift_down();
        move8.shift_left();
        move8.shift_left();
        moves |= move8;
    }

    moves
}
