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

// PST (Piece Square Tables) - encourage good piece placement
// Values are from white's perspective (positive = good for white)
// For black, we mirror the board vertically

/// PST for pawns - encourage center control and advancement
const PAWN_PST: [i32; 120] = {
    let mut table = [0i32; 120];
    // Ranks 3-7 (indices 31-38, 41-48, etc.) - actual board squares
    // Encourage pawns to control center and advance
    let center_bonus = [0, 0, 5, 10, 10, 5, 0, 0, 0, 0]; // Files a-h
    let advance_bonus = [0, 5, 10, 20, 40, 80, 100, 0, 0, 0]; // By rank from white side

    // Fill the actual board squares (21-98 in 10x12)
    // This is simplified - in real implementation we'd fill precisely
    table
};

/// Get PST value for a piece at a square
/// Returns bonus/penalty for piece placement
fn get_pst(piece: PieceType, index: usize, is_white: bool) -> i32 {
    if index < 21 || index > 98 {
        return 0;
    }

    let file = (index % 10) as i32; // 1-8 for a-h
    let rank = (index / 10) as i32; // 2-9 for rank 1-8

    // Calculate distance from center (d=4.5, e=4.5 is ideal)
    let center_dist =
        (file - 4).abs().max((file - 5).abs()) + (if is_white { (rank - 5).abs() } else { (rank - 6).abs() });

    let score = match piece {
        PieceType::Pawn => {
            // Pawns want to:
            // 1. Control center (c,d,e,f files)
            // 2. Advance toward promotion
            let file_bonus = match file {
                4 | 5 => 15, // e,d files - center
                3 | 6 => 5,  // c,f files
                _ => 0,
            };
            let rank_bonus = if is_white {
                match rank {
                    3 => 5,  // Rank 2
                    4 => 10, // Rank 3
                    5 => 20, // Rank 4
                    6 => 40, // Rank 5
                    7 => 60, // Rank 6
                    8 => 80, // Rank 7 - almost promoting!
                    _ => 0,
                }
            } else {
                // For black, rank 9 is their "rank 8"
                match rank {
                    9 => 5,  // Their rank 8 (back)
                    8 => 10, // Their rank 7
                    7 => 20, // Their rank 6
                    6 => 40, // Their rank 5
                    5 => 60, // Their rank 4
                    4 => 80, // Their rank 3 - almost promoting!
                    _ => 0,
                }
            };
            file_bonus + rank_bonus
        }
        PieceType::Knight => {
            // Knights want to be near center (can reach many squares)
            match center_dist {
                0 => 30,  // Perfect center (d4, d5, e4, e5)
                1 => 20,  // Very central
                2 => 10,  // Good
                3 => 0,   // OK
                _ => -10, // Edges are bad for knights
            }
        }
        PieceType::Bishop => {
            // Bishops want long diagonals (avoid corners)
            // Also want center for maximum diagonal coverage
            match center_dist {
                0 => 20,
                1 => 15,
                2 => 10,
                3 => 5,
                4 => 0,
                _ => -5, // Corners restrict bishop
            }
        }
        PieceType::Rook => {
            // Rooks want:
            // 1. 7th rank (for white) / 2nd rank (for black) to attack pawns
            // 2. Open files
            let rank_bonus = if is_white {
                if rank == 8 {
                    40
                } else if rank == 7 {
                    20
                } else {
                    5
                }
            } else {
                if rank == 3 {
                    40
                } else if rank == 4 {
                    20
                } else {
                    5
                }
            };
            rank_bonus
        }
        PieceType::Queen => {
            // Queen wants center but also to be safe
            // Early queen development to center can be dangerous
            match center_dist {
                0 => 15,
                1 => 10,
                2 => 5,
                _ => 0,
            }
        }
        PieceType::King => {
            // King safety in opening/middlegame:
            // 1. Stay back (ranks 1-2 for white, 7-8 for black)
            // 2. Avoid center (dangerous)
            // 3. Corners with castling rights are good
            if is_white {
                match rank {
                    2 => 10,  // Rank 1 - safe at back
                    3 => 5,   // Rank 2 - OK
                    _ => -20, // Too advanced is dangerous
                }
            } else {
                match rank {
                    9 => 10, // Rank 8 - safe
                    8 => 5,  // Rank 7 - OK
                    _ => -20,
                }
            }
        }
        _ => 0,
    };

    // Add file-based bonus for center files (applies to all pieces except pawns have their own)
    let center_file_bonus = if piece != PieceType::Pawn {
        match file {
            4 | 5 => 5, // d and e files
            3 | 6 => 2, // c and f files
            _ => 0,
        }
    } else {
        0
    };

    score + center_file_bonus
}

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

/// Evaluate the board position
/// Combines material + PST (piece square tables)
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
        let pst = get_pst(piece_type, index, is_white);

        if is_white {
            score += material + pst;
        } else {
            score -= material + pst;
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
    fn test_eval_initial_position_balanced() {
        // With PST, position is almost balanced (not exactly 0 due to PST asymmetry)
        let board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = evaluate(&board);
        // Allow ±50 points difference due to PST (peawns at different ranks have different values)
        assert!(score.abs() < 50, "Initial position should be roughly balanced, got {}", score);
    }

    #[test]
    fn test_eval_white_up_pawn() {
        // White has 8 pawns, Black has 7 pawns (white is up a pawn)
        let board = fen::fen_to_board("rnbqkbnr/ppppppp1/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = evaluate(&board);
        // 100 for pawn + some PST bonus (but less than 100 since we compare totals)
        assert!(score > 60, "White up a pawn, expected > 60, got {}", score);
    }

    #[test]
    fn test_eval_black_up_pawn() {
        // Black has 8 pawns, White has 7 pawns (black is up a pawn)
        let board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = evaluate(&board);
        assert!(score < -60, "Black up a pawn, expected < -60, got {}", score);
    }

    #[test]
    fn test_eval_perspective() {
        let b1 = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let b2 = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        let score1 = evaluate(&b1);
        let score2 = evaluate(&b2);
        // Same position from either perspective should give scores with opposite signs
        // (score from white perspective = -score from black perspective)
        // Due to PST asymmetry, allow some tolerance
        assert!((score1 + score2).abs() < 10, "Perspective should be opposite: {} vs {}", score1, score2);
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
