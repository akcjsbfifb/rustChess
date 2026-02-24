mod board;
mod constants;
mod eval;
mod movegen;
mod search;

use board::Board;

use crate::movegen::generate_moves;

fn main() {
    let board = Board::new();
    board.print();
    let moves = generate_moves(&board);
    println!("{moves:?}");
    println!("Movimientos generados: {}", moves.len());
}
