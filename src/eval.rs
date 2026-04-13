use crate::board::types::{Color, PieceType};
use crate::board::Board;
use crate::constants::*;

// Material values (centipawns)
pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 320;
pub const BISHOP_VALUE: i32 = 330;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 0;

/// Get material value for a piece type
pub fn piece_value(piece: PieceType) -> i32 {
    match piece {
        PieceType::Pawn => PAWN_VALUE,
        PieceType::Knight => KNIGHT_VALUE,
        PieceType::Bishop => BISHOP_VALUE,
        PieceType::Rook => ROOK_VALUE,
        PieceType::Queen => QUEEN_VALUE,
        PieceType::King => KING_VALUE,
        PieceType::None => 0,
    }
}

/// Evaluate the board position (material only, for now)
/// Returns positive score if better for side to move
pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0i32;

    for index in 0..BOARD_SIZE {
        let piece_byte = board.squares[index];
        if piece_byte == EMPTY || piece_byte == OFF_BOARD {
            continue;
        }

        let piece_type = board.get_piece_type(index);
        if piece_type == PieceType::None {
            continue;
        }

        let is_white = (piece_byte & COLOR_BIT) == 0;
        let material = piece_value(piece_type);

        if is_white {
            score += material;
        } else {
            score -= material;
        }
    }

    if board.side_to_move == Color::Black {
        score = -score;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen;

    #[test]
    fn test_eval_initial_position_is_zero() {
        let board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(evaluate(&board), 0);
    }

    #[test]
    fn test_eval_white_up_pawn() {
        // White has 8 pawns, Black has 7 pawns (white is up a pawn)
        let board = fen::fen_to_board("rnbqkbnr/ppppppp1/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = evaluate(&board);
        assert!(score > 80, "White up a pawn, expected > 80, got {}", score);
    }

    #[test]
    fn test_eval_black_up_pawn() {
        // Black has 8 pawns, White has 7 pawns (black is up a pawn)
        let board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = evaluate(&board);
        assert!(score < -80, "Black up a pawn, expected < -80, got {}", score);
    }

    #[test]
    fn test_eval_perspective() {
        let b1 = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let b2 = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        assert_eq!(evaluate(&b1), 0);
        assert_eq!(evaluate(&b2), 0);
    }

    #[test]
    fn test_eval_material_values() {
        assert_eq!(piece_value(PieceType::Pawn), 100);
        assert_eq!(piece_value(PieceType::Knight), 320);
        assert_eq!(piece_value(PieceType::Bishop), 330);
        assert_eq!(piece_value(PieceType::Rook), 500);
        assert_eq!(piece_value(PieceType::Queen), 900);
        assert_eq!(piece_value(PieceType::King), 0);
    }
}
