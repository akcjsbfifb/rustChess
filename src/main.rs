use rustChess::board::Board;
use rustChess::board::types::Color;
use rustChess::movegen::generate_moves;
use std::io::{self, Write};

fn main() {
    let mut board = Board::new();

    loop {
        board.print();
        let mut moves = generate_moves(&board);

        if moves.is_empty() {
            println!("No hay movimientos disponibles. Fin del juego.");
            break;
        }

        println!("\nMovimientos posibles ({}):", moves.len());
        for (i, m) in moves.iter().enumerate() {
            println!("{}: {:?}", i, m);
        }

        if board.side_to_move == Color::Black {
            // Machine: pick random move
            use std::time::{SystemTime, UNIX_EPOCH};
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as usize;
            let index = seed % moves.len();
            println!("\nMáquina (negras) juega: {}", index);
            board.make_move(moves[index]);
        } else {
            // User: read move index
            print!("\nElige un movimiento (índice): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error al leer entrada.");
                continue;
            }

            match input.trim().parse::<usize>() {
                Ok(index) if index < moves.len() => {
                    println!("Jugaste: {}", index);
                    board.make_move(moves[index]);
                }
                Ok(_) => {
                    println!("Índice inválido.");
                    continue;
                }
                Err(_) => {
                    println!("Entrada inválida.");
                    continue;
                }
            }
        }
    }
}
