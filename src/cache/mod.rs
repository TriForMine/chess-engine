use crate::bitboard::BitBoard;
use crate::coord::Coord;
use crate::r#move::Move;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod rook;

pub fn bitboard_to_moves(from: Coord, bitboard: BitBoard, is_capture: bool) -> Vec<Move> {
    let mut moves = Vec::new();

    for i in 0..64 {
        if bitboard.get_index(i) {
            moves.push(Move::new(from, Coord::from_index(i), is_capture, false));
        }
    }

    moves
}
