use crate::board::Board;
use crate::piece::PieceEnum;
use crate::r#move::Move;
use std::collections::HashMap;

pub struct TranspositionTable {
    entries: HashMap<(String, u32), i16>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, key: &(String, u32)) -> Option<i16> {
        self.entries.get(key).copied()
    }

    pub fn set(&mut self, key: (String, u32), value: i16) {
        self.entries.insert(key, value);
    }
}

pub struct Engine {
    pub board: Board,
    tt: TranspositionTable,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            board: Board::new_game(),
            tt: TranspositionTable::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Engine {
        Engine {
            board: Board::from_fen(fen).unwrap(),
            tt: TranspositionTable::new(),
        }
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        self.board.get_all_moves()
    }

    fn negamax(&mut self, depth: u32, alpha: i16, beta: i16) -> i16 {
        if depth == 0 {
            return self.board.evaluate();
        }

        let mut alpha = alpha;
        let mut max = -9999;

        let key = (self.board.to_fen(), depth);
        if let Some(value) = self.tt.get(&key) {
            return value;
        }

        for m in self.get_all_moves() {
            let fen = self.board.to_fen();
            self.board.make_move(m);

            let score = -self.negamax(depth - 1, -beta, -alpha);

            self.board.undo_move(m);

            max = max.max(score);

            alpha = alpha.max(score);
            if alpha >= beta {
                break;
            }
        }

        // Store the result in the transposition table
        self.tt.set(key, max);

        max
    }

    pub fn get_best_move(&mut self, depth: u32) -> Move {
        let mut best_move_value = -9999;
        let mut best_move = Move::null();

        for m in self.get_all_moves() {
            let fen = self.board.to_fen();
            self.board.make_move(m); // Changing the actual board state

            let move_value = -self.negamax(depth - 1, -10000, 10000);

            self.board.undo_move(m);

            if move_value > best_move_value {
                best_move_value = move_value;
                best_move = m;
            }
        }
        best_move
    }

    pub fn run(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "uci" {
            self.run_uci();
        } else if input == "play" {
            self.play();
        }
    }

    pub fn play(&mut self) {
        let depth = 3;

        while !self.board.is_game_over() {
            let m = self.get_best_move(depth);

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

    pub fn run_uci(&mut self) {
        println!("id name ChessEngine");
        println!("id author TriForMine");
        println!("uciok");

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            if input == "isready" {
                println!("readyok");
            } else if input == "ucinewgame" {
                self.board = Board::new_game();
            } else if input == "quit" {
                break;
            } else if input.starts_with("position") {
                let mut input = input.split_whitespace();

                input.next();

                if let Some(pos) = input.next() {
                    if pos == "startpos" {
                        self.board = Board::new_game();
                    } else {
                        let mut fen = String::new();

                        for _ in 0..6 {
                            fen.push_str(&format!("{} ", input.next().unwrap()));
                        }

                        fen.pop();

                        self.board.load_fen(&fen);
                    }
                }

                if let Some(m) = input.next() {
                    if m == "moves" {
                        while let Some(m) = input.next() {
                            let m = Move::from_str(m);
                            self.board.make_move(m);
                        }
                    }
                }
            } else if input.starts_with("go") {
                let mut input = input.split_whitespace();

                input.next();

                let mut depth = 3;

                while let Some(arg) = input.next() {
                    if arg == "depth" {
                        depth = input.next().unwrap().parse().unwrap();
                    }
                }

                let m = self.get_best_move(depth);

                println!("bestmove {}", m.to_str());
            }
        }
    }
}
