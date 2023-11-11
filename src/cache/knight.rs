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
    let move1 = knight_bitboard.shift_up().shift_up().shift_right();
    let move2 = knight_bitboard.shift_up().shift_up().shift_left();
    let move3 = knight_bitboard.shift_down().shift_down().shift_right();
    let move4 = knight_bitboard.shift_down().shift_down().shift_left();
    let move5 = knight_bitboard.shift_up().shift_right().shift_right();
    let move6 = knight_bitboard.shift_up().shift_left().shift_left();
    let move7 = knight_bitboard.shift_down().shift_right().shift_right();
    let move8 = knight_bitboard.shift_down().shift_left().shift_left();
    move1 | move2 | move3 | move4 | move5 | move6 | move7 | move8
}
