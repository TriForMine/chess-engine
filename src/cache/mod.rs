use crate::bitboard::BitBoard;
use crate::r#move::Move;

pub mod knight;

pub fn bitboard_to_moves(bitboard: BitBoard) -> Vec<Move> {
    let mut moves = Vec::new();

    for i in 0..64 {
        if bitboard.get_index(i) {
            moves.push(Move::from_bitboard(BitBoard::from_index(i), false, false));
        }
    }

    moves
}
