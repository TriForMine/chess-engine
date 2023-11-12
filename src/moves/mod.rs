use crate::bitboard::BitBoard;
use crate::board::Board;
use crate::cache::bishop::get_bishop_moves;
use crate::cache::bitboard_to_moves;
use crate::cache::king::KING_MOVES;
use crate::cache::knight::KNIGHT_MOVES;
use crate::cache::pawn::{PAWN_ATTACKS, PAWN_MOVES};
use crate::cache::rook::get_rook_moves;
use crate::coord::Coord;
use crate::r#move::Move;

pub fn filter_check_moves(board: &Board, moves: Vec<Move>) -> Vec<Move> {
    let mut filtered_moves = Vec::new();
    for m in moves {
        let mut board_copy = board.clone();
        board_copy.make_move(m);
        if !board_copy.is_check(board.turn) {
            filtered_moves.push(m);
        }
    }

    filtered_moves
}

pub fn generate_knights_moves(board: &Board, coord: Coord, color: bool) -> Vec<Move> {
    let knight_moves_bitboard = KNIGHT_MOVES[coord.to_index() as usize];
    let legal_moves_bitboard = knight_moves_bitboard & !board.occupied_bitboard();
    let legal_attacks_bitboard = knight_moves_bitboard & !board.get_piece_bitboard(color);

    let mut moves = bitboard_to_moves(coord, knight_moves_bitboard & legal_moves_bitboard, false);
    moves.append(&mut bitboard_to_moves(
        coord,
        knight_moves_bitboard & legal_attacks_bitboard,
        true,
    ));

    moves
}

pub fn generate_pawn_moves(board: &Board, coord: Coord, color: bool) -> Vec<Move> {
    let pawn_moves_bitboard = PAWN_MOVES[color as usize][coord.to_index() as usize];
    let legal_moves_bitboard = pawn_moves_bitboard & !board.occupied_bitboard();

    let pawn_attacks_bitboard =
        PAWN_ATTACKS[color as usize][coord.to_index() as usize] & board.get_enemy_bitboard(color);
    let legal_attacks_bitboard = pawn_attacks_bitboard & !board.get_piece_bitboard(color);

    let mut moves = bitboard_to_moves(coord, pawn_moves_bitboard & legal_moves_bitboard, false);

    moves.append(&mut bitboard_to_moves(
        coord,
        pawn_attacks_bitboard & legal_attacks_bitboard,
        true,
    ));

    moves
}

pub fn generate_bishop_moves(board: &Board, coord: Coord, color: bool) -> Vec<Move> {
    let bishop_moves_bitboard =
        get_bishop_moves(BitBoard::from_coord(coord), board.occupied_bitboard());

    let legal_moves_bitboard = bishop_moves_bitboard & !board.occupied_bitboard();
    let legal_attacks_bitboard = bishop_moves_bitboard & !board.get_piece_bitboard(color);

    // Apply blockers
    let mut moves = bitboard_to_moves(coord, bishop_moves_bitboard & legal_moves_bitboard, false);

    moves.append(&mut bitboard_to_moves(
        coord,
        bishop_moves_bitboard & legal_attacks_bitboard,
        true,
    ));

    moves
}

pub fn generate_rook_moves(board: &Board, coord: Coord, color: bool) -> Vec<Move> {
    let rook_moves_bitboard =
        get_rook_moves(BitBoard::from_coord(coord), board.occupied_bitboard());

    let legal_moves_bitboard = rook_moves_bitboard & !board.occupied_bitboard();
    let legal_attacks_bitboard = rook_moves_bitboard & !board.get_piece_bitboard(color);

    // Apply blockers
    let mut moves = bitboard_to_moves(coord, rook_moves_bitboard & legal_moves_bitboard, false);

    moves.append(&mut bitboard_to_moves(
        coord,
        rook_moves_bitboard & legal_attacks_bitboard,
        true,
    ));

    moves
}

pub fn generate_queen_moves(board: &Board, coord: Coord, color: bool) -> Vec<Move> {
    let mut moves = generate_bishop_moves(board, coord, color);
    moves.append(&mut generate_rook_moves(board, coord, color));
    moves
}

pub fn generate_king_moves(board: &Board, coord: Coord, color: bool) -> Vec<Move> {
    let king_moves_bitboard = KING_MOVES[coord.to_index() as usize];
    let legal_moves_bitboard = king_moves_bitboard & !board.occupied_bitboard();
    let legal_attacks_bitboard = king_moves_bitboard & !board.get_piece_bitboard(color);

    let mut moves = bitboard_to_moves(coord, king_moves_bitboard & legal_moves_bitboard, false);
    moves.append(&mut bitboard_to_moves(
        coord,
        king_moves_bitboard & legal_attacks_bitboard,
        true,
    ));

    moves
}
