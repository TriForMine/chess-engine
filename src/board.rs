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

#[derive(Clone)]
pub struct Board {
    pub white: OneSideBoard,
    pub black: OneSideBoard,
    pub turn: bool,
    pub previous_fen: Option<String>,
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
            previous_fen: None,
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

        if piece.is_white() == self.turn && self.is_check(self.turn) {
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
        let piece = self.get_piece(m.from).unwrap();
        self.unset_piece(m.from, piece);

        let captured_piece = self.get_piece(m.to);
        if let Some(captured_piece) = captured_piece {
            self.unset_piece(m.to, captured_piece);
        }

        self.set_piece(m.to, piece);
        self.turn = !self.turn;
    }

    pub fn is_check(&self, color: bool) -> bool {
        if let Some(king_coord) = self.get_piece_index_coord(PieceEnum::King.to_index()) {
            let enemy_moves = self.get_all_color_moves(!color);
            for m in enemy_moves {
                if m.to == king_coord {
                    return true;
                }
            }

            return false;
        }

        true
    }

    pub fn is_checkmate(&self, color: bool) -> bool {
        self.is_check(color) && self.get_all_moves().is_empty()
    }

    pub fn is_stalemate(&self, color: bool) -> bool {
        !self.is_check(color) && self.get_all_moves().is_empty()
    }

    pub fn is_draw(&self, color: bool) -> bool {
        self.is_stalemate(color) || self.is_checkmate(color)
    }

    pub fn is_game_over(&self) -> bool {
        self.is_draw(true) || self.is_draw(false)
    }

    pub fn evaluate(&self) -> i16 {
        let mut score: i16 = 0;

        for i in 0..64 {
            let coord = Coord::from_index(i);
            if self.has_piece(coord) {
                let piece = self.get_piece(coord).unwrap();

                score += piece.get_score(coord, self.turn);
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

    pub fn get_piece_index_coord(&self, index: u8) -> Option<Coord> {
        self.get_current_player().get_piece_index_coord(index)
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
