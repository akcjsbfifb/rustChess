use crate::board::Board;
use crate::board::types::{Color, Move, PieceType};
use crate::constants::*;

/// Genera todos los movimientos pseudo-legales para la posición actual.
pub fn generate_moves(board: &Board) -> Vec<Move> {
    let mut moves = Vec::with_capacity(256);
    // 1. Iterar por el tablero
    // 2. Identificar piezas del color side_to_move
    // 3. Llamar a los generadores específicos
    for index in 0..BOARD_SIZE {
        if board.is_color(index, board.side_to_move) {
            let piece_type = board.get_piece_type(index);
            match piece_type {
                PieceType::Pawn => {
                    generate_pawn_moves(board, index, board.side_to_move, &mut moves)
                }
                // PieceType::Knight => generate_stepper_moves(board, index, &[21, 19, 12, 8, -21, -19, -12, -8], &mut moves),
                // PieceType::Bishop => generate_slider_moves(board, index, &[11, 9, -11, -9], &mut moves),
                // PieceType::Rook => generate_slider_moves(board, index, &[10, -10, 1, -1], &mut moves),
                // PieceType::Queen => generate_slider_moves(board, index, &[10, -10, 1, -1, 11, 9, -11, -9], &mut moves),
                // PieceType::King => generate_stepper_moves(board, index, &[10, -10, 1, -1, 11, 9, -11, -9], &mut moves),
                _ => {}
            }
        }
    }
    moves
}
fn generate_pawn_moves(board: &Board, from: usize, side: Color, moves: &mut Vec<Move>) {
    // Lógica para avances, capturas, doble avance y promoción
    match side {
        Color::White => {
            // Avance hacia arriba (de menor a mayor índice)
            if board.squares[from + 10] == EMPTY {
                // Avance simple o promocion
                if from >= 81 && from <= 88 {
                    // Promoción por avance
                    add_promotion_moves(from, from + 10, PieceType::None, moves);
                } else {
                    moves.push(Move::new(
                        from,
                        from + 10,
                        PieceType::Pawn,
                        PieceType::None,
                        PieceType::None,
                        Move::FLAG_NONE,
                    ));
                }
                if from >= 31 && from <= 38 && board.squares[from + 20] == EMPTY {
                    // Doble avance
                    moves.push(Move::new(
                        from,
                        from + 20,
                        PieceType::Pawn,
                        PieceType::None,
                        PieceType::None,
                        Move::FLAG_DOUBLE_PAWN,
                    ));
                }
            }
            if board.is_color(from + 9, Color::Black) {
                // Captura diagonal derecha
                let captured = board.get_piece_type(from + 9);
                if from >= 81 && from <= 88 {
                    // Promoción con captura
                    add_promotion_moves(from, from + 9, captured, moves);
                } else {
                    moves.push(Move::new(
                        from,
                        from + 9,
                        PieceType::Pawn,
                        captured,
                        PieceType::None,
                        Move::FLAG_NONE,
                    ));
                }
            }
            if board.is_color(from + 11, Color::Black) {
                // Captura diagonal izquierda
                let captured = board.get_piece_type(from + 11);
                if from >= 81 && from <= 88 {
                    // Promoción con captura
                    add_promotion_moves(from, from + 11, captured, moves);
                } else {
                    moves.push(Move::new(
                        from,
                        from + 11,
                        PieceType::Pawn,
                        captured,
                        PieceType::None,
                        Move::FLAG_NONE,
                    ));
                }
            }
        }
        Color::Black => {
            // Avance hacia abajo (de mayor a menor índice)
            if board.squares[from - 10] == EMPTY {
                // Avance simple
                if from >= 31 && from <= 38 {
                    // Promoción por avance
                    add_promotion_moves(from, from - 10, PieceType::None, moves);
                } else {
                    moves.push(Move::new(
                        from,
                        from - 10,
                        PieceType::Pawn,
                        PieceType::None,
                        PieceType::None,
                        Move::FLAG_NONE,
                    ));
                }
                if from >= 81 && from <= 88 && board.squares[from - 20] == EMPTY {
                    // Doble avance
                    moves.push(Move::new(
                        from,
                        from - 20,
                        PieceType::Pawn,
                        PieceType::None,
                        PieceType::None,
                        Move::FLAG_DOUBLE_PAWN,
                    ));
                }
            }
            if board.is_color(from - 9, Color::White) {
                // Captura diagonal derecha
                let captured = board.get_piece_type(from - 9);
                if from >= 31 && from <= 38 {
                    // Promoción con captura
                    add_promotion_moves(from, from - 9, captured, moves);
                } else {
                    moves.push(Move::new(
                        from,
                        from - 9,
                        PieceType::Pawn,
                        captured,
                        PieceType::None,
                        Move::FLAG_NONE,
                    ));
                }
            }
            if board.is_color(from - 11, Color::White) {
                // Captura diagonal izquierda
                let captured = board.get_piece_type(from - 11);
                if from >= 31 && from <= 38 {
                    // Promoción con captura
                    add_promotion_moves(from, from - 11, captured, moves);
                } else {
                    moves.push(Move::new(
                        from,
                        from - 11,
                        PieceType::Pawn,
                        captured,
                        PieceType::None,
                        Move::FLAG_NONE,
                    ));
                }
            }
        }
    }
}

fn add_promotion_moves(from: usize, to: usize, captured: PieceType, moves: &mut Vec<Move>) {
    moves.push(Move::new(
        from,
        to,
        PieceType::Pawn,
        captured,
        PieceType::Queen,
        Move::FLAG_NONE,
    ));
    moves.push(Move::new(
        from,
        to,
        PieceType::Pawn,
        captured,
        PieceType::Rook,
        Move::FLAG_NONE,
    ));
    moves.push(Move::new(
        from,
        to,
        PieceType::Pawn,
        captured,
        PieceType::Bishop,
        Move::FLAG_NONE,
    ));
    moves.push(Move::new(
        from,
        to,
        PieceType::Pawn,
        captured,
        PieceType::Knight,
        Move::FLAG_NONE,
    ));
}
fn generate_stepper_moves(board: &Board, from: usize, offsets: &[isize], moves: &mut Vec<Move>) {
    // Lógica para piezas que no deslizan (Caballo, Rey)
}
fn generate_slider_moves(board: &Board, from: usize, offsets: &[isize], moves: &mut Vec<Move>) {
    // Lógica para piezas que deslizan (Torre, Alfil, Dama)
}
/// Verifica si un cuadro está siendo atacado por un color específico.
pub fn is_square_attacked(board: &Board, square: usize, by_color: Color) -> bool {
    // Se usa para detectar jaques y validar enroques
    false
}
