use crate::bitboard::BitBoard;
use crate::coord::Coord;
use crate::moves::{
    filter_check_moves, generate_bishop_moves, generate_king_moves, generate_knights_moves,
    generate_pawn_moves, generate_queen_moves, generate_rook_moves,
};
use crate::piece::{Piece, PieceEnum};
use crate::r#move::Move;
use colored::Colorize;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub struct CheckState {
    pub is_check: bool,
    pub is_checkmate: bool,
    pub is_stalemate: bool,
    pub is_draw: bool,
}

#[derive(Clone)]
pub struct MoveWithCapture {
    pub m: Move,
    pub captured: Option<Piece>,
}

#[derive(Clone)]
pub struct Board {
    pub white: OneSideBoard,
    pub black: OneSideBoard,
    pub turn: bool,
    past_moves: Vec<MoveWithCapture>,

    pub check_states: [CheckState; 2],
}

#[derive(Clone)]
pub struct OneSideBoard {
    pieces: [BitBoard; 6],
    all_pieces: BitBoard,
    color: bool,
}

impl Board {
    pub const STARTING_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w";

    pub fn new() -> Board {
        Board {
            white: OneSideBoard::new(true),
            black: OneSideBoard::new(false),
            turn: true,
            past_moves: Vec::new(),
            check_states: [
                CheckState {
                    is_check: false,
                    is_checkmate: false,
                    is_stalemate: false,
                    is_draw: false,
                },
                CheckState {
                    is_check: false,
                    is_checkmate: false,
                    is_stalemate: false,
                    is_draw: false,
                },
            ],
        }
    }

    pub fn new_game() -> Board {
        Board::from_fen(Board::STARTING_FEN).unwrap()
    }

    pub fn get_current_player(&self) -> &OneSideBoard {
        if self.turn {
            &self.white
        } else {
            &self.black
        }
    }

    pub fn get_moves(&self, coord: Coord) -> Vec<Move> {
        let piece = self.get_piece(coord).unwrap();
        let moves = match piece.piece {
            PieceEnum::Pawn => generate_pawn_moves(self, coord, piece.is_white()),
            PieceEnum::Knight => generate_knights_moves(self, coord, piece.is_white()),
            PieceEnum::Bishop => generate_bishop_moves(self, coord, piece.is_white()),
            PieceEnum::Rook => generate_rook_moves(self, coord, piece.is_white()),
            PieceEnum::Queen => generate_queen_moves(self, coord, piece.is_white()),
            PieceEnum::King => generate_king_moves(self, coord, piece.is_white()),
        };

        if piece.is_white() == self.turn && self.check_states[self.turn as usize].is_check {
            filter_check_moves(self, moves)
        } else {
            moves
        }
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        self.get_all_color_moves(self.turn)
    }

    pub fn get_all_enemy_moves(&self) -> Vec<Move> {
        self.get_all_color_moves(!self.turn)
    }

    pub fn get_all_color_moves(&self, color: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        let bitboard = self.get_piece_bitboard(color);
        for i in 0..64 {
            let coord = Coord::from_index(i);
            if bitboard.get(coord) {
                moves.extend(self.get_moves(coord));
            }
        }
        moves
    }

    pub fn make_move(&mut self, m: Move) {
        let src_piece = self.get_piece(m.from).unwrap();
        let captured_piece = self.get_piece(m.to);

        self.unset_piece(m.from, src_piece);

        if let Some(captured) = captured_piece {
            self.unset_piece(m.to, captured);
        }

        self.set_piece(m.to, src_piece);

        self.past_moves.push(MoveWithCapture {
            m,
            captured: captured_piece,
        });
        self.turn = !self.turn;

        self.update_flags();
    }

    pub fn undo_move(&mut self, m: Move) {
        if let Some(move_with_capture) = self.past_moves.pop() {
            assert_eq!(m, move_with_capture.m);

            let moved_piece = self.get_piece(m.to).unwrap();
            self.unset_piece(m.to, moved_piece);
            self.set_piece(m.from, moved_piece);

            if let Some(captured) = move_with_capture.captured {
                self.set_piece(m.to, captured);
            }

            self.turn = !self.turn;

            self.update_flags();
        } else {
            eprintln!("No moves to undo");
        }
    }

    pub fn update_flags(&mut self) {
        let moves = self.get_all_moves();
        let is_empty = moves.is_empty();

        self.check_states[self.turn as usize].is_check = self.calculate_is_check(self.turn);
        self.check_states[self.turn as usize].is_checkmate =
            self.calculate_is_checkmate(self.turn, is_empty);
        self.check_states[self.turn as usize].is_stalemate =
            self.calculate_is_stalemate(self.turn, is_empty);
        self.check_states[self.turn as usize].is_draw = self.calculate_is_draw(self.turn);
    }

    pub fn calculate_is_check(&self, color: bool) -> bool {
        if let Some(king_coord) = self.get_piece_index_coord(5, color) {
            let enemy_moves = self.get_all_enemy_moves();
            enemy_moves.iter().any(|m| m.to == king_coord)
        } else {
            true
        }
    }

    pub fn calculate_is_checkmate(&self, color: bool, is_empty: bool) -> bool {
        self.check_states[color as usize].is_check && is_empty
    }

    pub fn calculate_is_stalemate(&self, color: bool, is_empty: bool) -> bool {
        !self.check_states[color as usize].is_check && is_empty
    }

    pub fn calculate_is_draw(&self, color: bool) -> bool {
        self.check_states[color as usize].is_stalemate || self.check_states[color as usize].is_draw
    }

    pub fn is_check(&self, color: bool) -> bool {
        self.check_states[color as usize].is_check
    }

    pub fn is_checkmate(&self, color: bool) -> bool {
        self.check_states[color as usize].is_checkmate
    }

    pub fn is_stalemate(&self, color: bool) -> bool {
        self.check_states[color as usize].is_stalemate
    }

    pub fn is_draw(&self, color: bool) -> bool {
        self.check_states[color as usize].is_draw
    }

    pub fn is_game_over(&self) -> bool {
        self.is_checkmate(self.turn) || self.is_stalemate(self.turn)
    }

    pub fn evaluate(&self) -> i16 {
        let mut score: i16 = 0;

        for i in 0..64 {
            let coord = Coord::from_index(i);

            if let Some(piece) = self.get_piece(coord) {
                if piece.is_white() {
                    score += piece.get_score(coord);
                } else {
                    score -= piece.get_score(coord);
                }
            }
        }

        score
    }

    pub fn get_piece(&self, coord: Coord) -> Option<Piece> {
        self.white
            .get_piece(coord)
            .or_else(|| self.black.get_piece(coord))
    }

    pub fn get_piece_index(&self, index: u8) -> Option<Piece> {
        self.get_current_player().get_piece_index(index)
    }

    pub fn get_piece_index_coord(&self, index: u8, color: bool) -> Option<Coord> {
        if color {
            self.white.get_piece_index_coord(index)
        } else {
            self.black.get_piece_index_coord(index)
        }
    }

    pub fn get_piece_bitboard(&self, color: bool) -> BitBoard {
        if color {
            self.white.all_pieces
        } else {
            self.black.all_pieces
        }
    }

    pub fn get_enemy_bitboard(&self, color: bool) -> BitBoard {
        if color {
            self.black.all_pieces
        } else {
            self.white.all_pieces
        }
    }

    pub fn set_piece(&mut self, coord: Coord, piece: Piece) {
        if piece.is_white() {
            self.white.set_piece(coord, piece);
        } else {
            self.black.set_piece(coord, piece);
        }
    }

    pub fn unset_piece(&mut self, coord: Coord, piece: Piece) {
        if piece.is_white() {
            self.white.unset_piece(coord, piece);
        } else {
            self.black.unset_piece(coord, piece);
        }
    }

    pub fn has_piece(&self, coord: Coord) -> bool {
        self.white.has_piece(coord) || self.black.has_piece(coord)
    }

    pub fn is_white(&self, coord: Coord) -> bool {
        self.white.has_piece(coord)
    }

    pub fn is_black(&self, coord: Coord) -> bool {
        self.black.has_piece(coord)
    }

    pub fn is_empty(&self, coord: Coord) -> bool {
        !self.has_piece(coord)
    }

    pub fn is_opponent(&self, coord: Coord) -> bool {
        if self.turn {
            self.is_black(coord)
        } else {
            self.is_white(coord)
        }
    }

    pub fn occupied_bitboard(&self) -> BitBoard {
        self.white.all_pieces | self.black.all_pieces
    }

    pub fn load_fen(&mut self, fen: &str) {
        let board = Board::from_fen(fen).unwrap();
        self.white = board.white;
        self.black = board.black;
        self.turn = board.turn;
    }

    pub fn from_fen(fen: &str) -> Option<Board> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let mut board = Board::new();
        let mut rank = 7;
        let mut file = 0;

        if let Some(piece_placement) = parts.get(0) {
            for c in piece_placement.chars() {
                if c == '/' {
                    rank -= 1;
                    file = 0;
                } else if c.is_digit(10) {
                    file += c.to_digit(10)? as u8;
                } else {
                    let piece = Piece::from_char(c)?;
                    let coord = Coord::new(file, rank);
                    board.set_piece(coord, piece);
                    file += 1;
                }
            }
        }

        if let Some(turn) = parts.get(1) {
            board.turn = if *turn == "w" { true } else { false };
        }

        Some(board)
    }

    pub fn to_fen(&self) -> String {
        let is_checkmate = self.is_checkmate(self.turn);
        let is_stalemate = self.is_stalemate(self.turn);

        let mut fen = String::with_capacity(50);
        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                let coord = Coord::new(file, rank);
                if let Some(piece) = self.get_piece(coord) {
                    // Using `if let` to avoid unwrap
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }
        fen.push(' ');
        fen.push(if self.turn { 'w' } else { 'b' });

        fen
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let moves = self.get_all_moves();
        let capturable_coords: Vec<Coord> = moves.into_iter().map(|m| m.to).collect();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let coord = Coord::new(file, rank);
                if let Some(piece) = self.get_piece(coord) {
                    let piece_char = piece.to_char();
                    if capturable_coords.contains(&coord) {
                        s.push_str(&piece_char.to_string().red().bold().to_string());
                    } else {
                        let formatted_piece = match piece.is_white() {
                            true => piece_char.to_string().white().bold().to_string(),
                            false => piece_char.to_string().black().bold().to_string(),
                        };

                        s.push_str(&formatted_piece);
                    }
                } else {
                    s.push_str(&".".to_string());
                }
            }
            s.push('\n');
        }

        if self.is_checkmate(self.turn) {
            s.push_str(&"Checkmate!\n".red().bold().to_string());
        }
        if self.is_stalemate(self.turn) {
            s.push_str(&"Stalemate!\n".yellow().bold().to_string());
        }

        write!(f, "{}", s)
    }
}

impl OneSideBoard {
    pub fn new(color: bool) -> OneSideBoard {
        OneSideBoard {
            pieces: [
                BitBoard::new(),
                BitBoard::new(),
                BitBoard::new(),
                BitBoard::new(),
                BitBoard::new(),
                BitBoard::new(),
            ],
            all_pieces: BitBoard::new(),
            color,
        }
    }

    pub fn get_piece(&self, coord: Coord) -> Option<Piece> {
        for i in 0..6 {
            if self.pieces[i].get(coord) {
                return Some(Piece::from_index(i as u8, self.color));
            }
        }

        None
    }

    pub fn get_piece_index(&self, index: u8) -> Option<Piece> {
        if self.pieces[index as usize].get_board() != 0 {
            Some(Piece::from_index(index, self.color))
        } else {
            None
        }
    }

    pub fn get_piece_index_coord(&self, index: u8) -> Option<Coord> {
        if self.pieces[index as usize].get_board() != 0 {
            Some(Coord::from_index(
                self.pieces[index as usize].get_board().trailing_zeros() as u8,
            ))
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, coord: Coord, piece: Piece) {
        self.pieces[piece.to_index() as usize].set(coord);
        self.all_pieces.set(coord);
    }

    pub fn unset_piece(&mut self, coord: Coord, piece: Piece) {
        self.pieces[piece.to_index() as usize].unset(coord);
        self.all_pieces.unset(coord);
    }

    pub fn has_piece(&self, coord: Coord) -> bool {
        self.all_pieces.get(coord)
    }
}

#[cfg(test)]
// Test evaluation

mod tests {
    use super::*;
    use crate::r#move::Move;

    #[test]
    fn test_score() {
        let board = Board::new_game();
        assert_eq!(board.evaluate(), 0);
    }
}
