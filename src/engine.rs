use crate::board::Board;
use crate::piece::PieceEnum;
use crate::r#move::Move;

pub struct Engine {
    pub board: Board,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            board: Board::new_game(),
        }
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        self.board.get_all_moves()
    }

    fn negamax(&mut self, depth: u32, alpha: i16, beta: i16) -> i16 {
        // Changed to mutable
        if depth == 0 || self.board.is_game_over() {
            return self.board.evaluate();
        }

        let mut alpha = alpha;
        let mut max = -9999;

        for m in self.get_all_moves() {
            let fen = self.board.to_fen();
            self.board.make_move(m); // Changing the actual board state
            let score = -self.negamax(depth - 1, -beta, -alpha);

            self.board.load_fen(&fen);

            max = max.max(score);

            alpha = alpha.max(score);
            if alpha >= beta {
                break;
            }
        }
        max
    }

    pub fn alpha_beta(&mut self, depth: u32) -> Move {
        // Changed to mutable
        let mut best_move_value = -9999;
        let mut best_move = Move::null();

        for m in self.get_all_moves() {
            let fen = self.board.to_fen();
            self.board.make_move(m); // Changing the actual board state
            let move_value = -self.negamax(depth - 1, -10000, 10000);

            self.board.load_fen(&fen);

            if move_value > best_move_value {
                best_move_value = move_value;
                best_move = m;
            }
        }
        best_move
    }

    pub fn play(&mut self) {
        let depth = 3;

        while !self.board.is_game_over() {
            let m = self.alpha_beta(depth);

            if m == Move::null() {
                println!("No move found");
                break;
            }

            if let Some(piece) = self.board.get_piece(m.to) {
                if piece.piece == PieceEnum::King {
                    println!("Checkmate!");
                    break;
                }
            }

            self.board.make_move(m);
            println!("{}", self.board);
        }
    }
}
