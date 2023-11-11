use crate::board::Board;
use crate::engine::Engine;

mod bitboard;
mod board;
mod cache;
mod coord;
mod engine;
mod evaluate;
mod r#move;
mod moves;
mod piece;

fn main() {
    let mut engine = Engine::new();

    let m = engine.play();
}
