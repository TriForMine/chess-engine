use crate::bitboard::{BitBoard, EMPTY};

static mut KNIGHT_MOVES: [BitBoard; 64] = [EMPTY; 64];

pub fn init_knight_moves() {
    for i in 0_u8..64_u8 {
        unsafe {
            KNIGHT_MOVES[i] = all_knight_moves(BitBoard::from_index(i));
        }
    }
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
