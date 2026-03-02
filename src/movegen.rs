use crate::board::types::{Color, Move, PieceType};
use crate::board::Board;
use crate::board::*;
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
                PieceType::Pawn => generate_pawn_moves(board, index, board.side_to_move, &mut moves),
                PieceType::Knight => {
                    generate_stepper_moves(board, index, &[21, 19, 12, 8, -21, -19, -12, -8], &mut moves)
                }
                PieceType::Bishop => generate_slider_moves(board, index, &[11, 9, -11, -9], &mut moves),
                PieceType::Rook => generate_slider_moves(board, index, &[10, -10, 1, -1], &mut moves),
                PieceType::Queen => generate_slider_moves(board, index, &[10, -10, 1, -1, 11, 9, -11, -9], &mut moves),
                PieceType::King => {
                    let (oo_flag, ooo_flag) = match board.side_to_move {
                        Color::White => (WHITE_OO, WHITE_OOO),
                        Color::Black => (BLACK_OO, BLACK_OOO),
                    };

                    if (board.can_castle & oo_flag) != 0 {
                        generate_castling_moves(board, index, 2, &mut moves);
                    }
                    if (board.can_castle & ooo_flag) != 0 {
                        generate_castling_moves(board, index, -2, &mut moves);
                    }

                    generate_stepper_moves(board, index, &[10, -10, 1, -1, 11, 9, -11, -9], &mut moves);
                }
                _ => {}
            }
        }
    }
    moves
}
fn generate_castling_moves(board: &Board, from: usize, offset: isize, moves: &mut Vec<Move>) {
    let opponent = board.side_to_move.opponent();

    if offset == 2 {
        if board.squares[from + 1] == EMPTY
            && board.squares[from + 2] == EMPTY
            && !is_square_attacked(board, from, opponent)
            && !is_square_attacked(board, from + 1, opponent)
            && !is_square_attacked(board, from + 2, opponent)
        {
            moves.push(Move::new(
                from,
                from + 2,
                PieceType::King,
                PieceType::None,
                PieceType::None,
                Move::FLAG_CASTLE_KING,
            ));
        }
    } else if offset == -2
        && board.squares[from - 1] == EMPTY
        && board.squares[from - 2] == EMPTY
        && board.squares[from - 3] == EMPTY
        && !is_square_attacked(board, from, opponent)
        && !is_square_attacked(board, from - 1, opponent)
        && !is_square_attacked(board, from - 2, opponent)
    {
        moves.push(Move::new(
            from,
            from - 2,
            PieceType::King,
            PieceType::None,
            PieceType::None,
            Move::FLAG_CASTLE_QUEEN,
        ));
    }
}
fn generate_pawn_moves(board: &Board, from: usize, side: Color, moves: &mut Vec<Move>) {
    // Lógica para avances, capturas, doble avance y promoción
    match side {
        Color::White => {
            // Avance hacia arriba (de menor a mayor índice)
            if board.squares[from + 10] == EMPTY {
                if (A7..=H7).contains(&from) {
                    // Promoción por avance
                    add_promotion_moves(from, from + 10, PieceType::None, moves);
                } else {
                    // Avance simple
                    moves.push(Move::new(
                        from,
                        from + 10,
                        PieceType::Pawn,
                        PieceType::None,
                        PieceType::None,
                        Move::FLAG_NONE,
                    ));
                }
                if (A2..=H2).contains(&from) && board.squares[from + 20] == EMPTY {
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
                let captured = board.get_piece_type(from + 9);
                if (A7..=H7).contains(&from) {
                    // Promoción con captura
                    add_promotion_moves(from, from + 9, captured, moves);
                } else {
                    moves.push(Move::new(from, from + 9, PieceType::Pawn, captured, PieceType::None, Move::FLAG_NONE));
                }
            }
            if board.is_color(from + 11, Color::Black) {
                let captured = board.get_piece_type(from + 11);
                if (A7..=H7).contains(&from) {
                    // Promoción con captura
                    add_promotion_moves(from, from + 11, captured, moves);
                } else {
                    moves.push(Move::new(from, from + 11, PieceType::Pawn, captured, PieceType::None, Move::FLAG_NONE));
                }
            }

            // En passant posible
            // Captura al paso
            if (A5..=H5).contains(&from)
                && let Some(index) = board.undo_info.last().unwrap().en_passant_square {
                    if index == from - 1 {
                        moves.push(Move::new(
                            from,
                            from + 9,
                            PieceType::Pawn,
                            PieceType::Pawn,
                            PieceType::None,
                            Move::FLAG_EN_PASSANT,
                        ));
                    } else if index == from + 1 {
                        moves.push(Move::new(
                            from,
                            from + 11,
                            PieceType::Pawn,
                            PieceType::Pawn,
                            PieceType::None,
                            Move::FLAG_EN_PASSANT,
                        ));
                    }
                }
        }
        Color::Black => {
            // Avance hacia abajo (de mayor a menor índice)
            if board.squares[from - 10] == EMPTY {
                // Avance simple
                if (A2..=H2).contains(&from) {
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
                if (A7..=H7).contains(&from) && board.squares[from - 20] == EMPTY {
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
                if (A2..=H2).contains(&from) {
                    // Promoción con captura
                    add_promotion_moves(from, from - 9, captured, moves);
                } else {
                    moves.push(Move::new(from, from - 9, PieceType::Pawn, captured, PieceType::None, Move::FLAG_NONE));
                }
            }
            if board.is_color(from - 11, Color::White) {
                // Captura diagonal izquierda
                let captured = board.get_piece_type(from - 11);
                if (A2..=H2).contains(&from) {
                    // Promoción con captura
                    add_promotion_moves(from, from - 11, captured, moves);
                } else {
                    moves.push(Move::new(from, from - 11, PieceType::Pawn, captured, PieceType::None, Move::FLAG_NONE));
                }
            }
            if (A4..=H4).contains(&from)
                && let Some(index) = board.undo_info.last().unwrap().en_passant_square {
                    if index == from + 1 {
                        // En passant posible
                        // Captura al paso
                        moves.push(Move::new(
                            from,
                            from - 9,
                            PieceType::Pawn,
                            PieceType::Pawn,
                            PieceType::None,
                            Move::FLAG_EN_PASSANT,
                        ));
                    } else if index == from - 1 {
                        // En passant posible
                        // Captura al paso
                        moves.push(Move::new(
                            from,
                            from - 11,
                            PieceType::Pawn,
                            PieceType::Pawn,
                            PieceType::None,
                            Move::FLAG_EN_PASSANT,
                        ));
                    }
                }
        }
    }
}

fn add_promotion_moves(from: usize, to: usize, captured: PieceType, moves: &mut Vec<Move>) {
    moves.push(Move::new(from, to, PieceType::Pawn, captured, PieceType::Queen, Move::FLAG_PROMOTION));
    moves.push(Move::new(from, to, PieceType::Pawn, captured, PieceType::Rook, Move::FLAG_PROMOTION));
    moves.push(Move::new(from, to, PieceType::Pawn, captured, PieceType::Bishop, Move::FLAG_PROMOTION));
    moves.push(Move::new(from, to, PieceType::Pawn, captured, PieceType::Knight, Move::FLAG_PROMOTION));
}
fn generate_stepper_moves(board: &Board, from: usize, offsets: &[isize], moves: &mut Vec<Move>) {
    // Caballo, Rey
    for &offset in offsets {
        let to = (from as isize + offset) as usize;
        if board.squares[to] == OFF_BOARD {
            continue;
        }
        if board.is_color(to, board.side_to_move) {
            continue;
        }
        let captured = if board.squares[to] == EMPTY { PieceType::None } else { board.get_piece_type(to) };
        moves.push(Move::new(from, to, board.get_piece_type(from), captured, PieceType::None, Move::FLAG_NONE));
    }
}
fn generate_slider_moves(board: &Board, from: usize, offsets: &[isize], moves: &mut Vec<Move>) {
    // Torre, Alfil, Dama
    for &offset in offsets {
        let mut to = (from as isize + offset) as usize;
        while board.squares[to] != OFF_BOARD {
            if board.is_color(to, board.side_to_move) {
                break;
            }
            let captured = if board.squares[to] == EMPTY { PieceType::None } else { board.get_piece_type(to) };
            moves.push(Move::new(from, to, board.get_piece_type(from), captured, PieceType::None, Move::FLAG_NONE));
            if captured != PieceType::None {
                break;
            }
            to = (to as isize + offset) as usize;
        }
    }
}
/// Verifica si un cuadro está siendo atacado por un color específico.
pub fn is_square_attacked(board: &Board, square: usize, by_color: Color) -> bool {
    // Se usa para detectar jaques y validar enroques
    if board.is_color(square, by_color) || board.squares[square] == OFF_BOARD {
        return false; // No puede estar atacado por su propio color
    }
    let knightoffsets = [21, 19, 12, 8, -21, -19, -12, -8];
    let queen_offsets = [10, -10, 1, -1, 11, 9, -11, -9];
    let pawn_offsets = match by_color {
        Color::White => [-9, -11], // Ataques de peones blancos
        Color::Black => [9, 11],   // Ataques de
    };
    for &offset in &knightoffsets {
        let to = (square as isize + offset) as usize;
        if board.squares[to] == OFF_BOARD {
            continue;
        }
        if board.is_color(to, by_color) && board.get_piece_type(to) == PieceType::Knight {
            return true;
        }
    }
    for &offset in &queen_offsets {
        let to = (square as isize + offset) as usize;

        if board.squares[to] == OFF_BOARD {
            continue;
        }
        if board.is_color(to, by_color) && board.get_piece_type(to) == PieceType::King {
            return true;
        }
    }
    for &offset in &queen_offsets {
        let mut to = (square as isize + offset) as usize;
        while board.squares[to] != OFF_BOARD {
            if board.is_color(to, by_color) {
                let piece_type = board.get_piece_type(to);
                if piece_type == PieceType::Queen
                    || (piece_type == PieceType::Rook && (offset.abs() == 1 || offset.abs() == 10))
                    || (piece_type == PieceType::Bishop && offset.abs() != 1 && offset.abs() != 10)
                {
                    return true;
                }
                break;
            }
            if board.squares[to] != EMPTY {
                break;
            }
            to = (to as isize + offset) as usize;
        }
    }
    for &offset in &pawn_offsets {
        let to = (square as isize + offset) as usize;
        if board.squares[to] == OFF_BOARD {
            continue;
        }
        if board.is_color(to, by_color) && board.get_piece_type(to) == PieceType::Pawn {
            return true;
        }
    }
    false
}

fn is_in_check(board: &Board) -> bool {
    let king_square =
        if board.side_to_move == Color::White { board.white_king_index } else { board.black_king_index };
    is_square_attacked(board, king_square, board.side_to_move.opponent())
}
pub fn perft(board: &mut Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_moves(board);
    let mut nodes = 0;

    for mv in moves {
        board.make_move(mv);
        if is_in_check(board) {
            board.unmake_move();
            continue; // Movimiento ilegal, no contar
        }
        nodes += perft(board, depth - 1);
        board.unmake_move();
    }

    nodes
}
