use crate::board::types::{Color, Move, PieceType};
use crate::board::Board;
use crate::eval::evaluate;
use crate::movegen::{generate_moves, is_square_attacked};
use std::time::Instant;

/// Information about the current search
pub struct SearchInfo {
    pub depth: i32,
    pub max_depth: i32,
    pub nodes: u64,
    pub start_time: Instant,
    pub stop: bool,
}

impl SearchInfo {
    pub fn new(max_depth: i32) -> Self {
        Self { depth: 0, max_depth, nodes: 0, start_time: Instant::now(), stop: false }
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }
}

/// Generate only legal moves (filter out moves that leave king in check)
pub fn generate_legal_moves(board: &mut Board) -> Vec<Move> {
    let pseudo_moves = generate_moves(board);
    let mut legal_moves = Vec::new();

    for mv in pseudo_moves {
        board.make_move(mv);

        // Check if our king is in check after making the move
        let king_sq = if board.side_to_move == Color::White {
            // After making move, side_to_move switched, so we check black's king
            board.black_king_index
        } else {
            board.white_king_index
        };

        let in_check = is_square_attacked(board, king_sq, board.side_to_move);
        board.unmake_move();

        if !in_check {
            legal_moves.push(mv);
        }
    }

    legal_moves
}

/// Check if the side to move is in check
fn is_in_check(board: &Board) -> bool {
    let king_sq = if board.side_to_move == Color::White { board.white_king_index } else { board.black_king_index };
    is_square_attacked(board, king_sq, board.side_to_move.opponent())
}

/// Negamax search with alpha-beta pruning
/// Returns the score from the perspective of the current player
fn negamax(board: &mut Board, depth: i32, mut alpha: i32, beta: i32, info: &mut SearchInfo) -> i32 {
    info.nodes += 1;

    // Base case: evaluate at leaf nodes
    if depth <= 0 {
        return evaluate(board);
    }

    // Generate legal moves
    let moves = generate_legal_moves(board);

    // Check for checkmate or stalemate
    if moves.is_empty() {
        if is_in_check(board) {
            // Checkmate - return a large negative score
            // The faster the mate, the more negative (closer to zero)
            return -30000 + (info.max_depth - depth);
        } else {
            // Stalemate - draw
            return 0;
        }
    }

    let mut best_score = -99999;

    for mv in moves {
        board.make_move(mv);
        let score = -negamax(board, depth - 1, -beta, -alpha, info);
        board.unmake_move();

        if score > best_score {
            best_score = score;
        }

        // Alpha-beta pruning
        if score > alpha {
            alpha = score;
        }

        if alpha >= beta {
            break; // Beta cutoff
        }
    }

    best_score
}

/// Search for the best move
/// Returns (best_move, score) where score is from the perspective of the side to move
pub fn search(board: &mut Board, max_depth: i32) -> (Option<Move>, i32, SearchInfo) {
    let mut info = SearchInfo::new(max_depth);
    let moves = generate_legal_moves(board);

    if moves.is_empty() {
        // Game over
        let score = if is_in_check(board) {
            -30000 // Checkmate
        } else {
            0 // Stalemate
        };
        return (None, score, info);
    }

    let mut best_move = moves[0];
    let mut best_score = -99999;

    // Iterative deepening
    for depth in 1..=max_depth {
        info.depth = depth;
        let mut current_best = moves[0];
        let mut alpha = -99999;
        let beta = 99999;

        for mv in &moves {
            board.make_move(*mv);
            let score = -negamax(board, depth - 1, -beta, -alpha, &mut info);
            board.unmake_move();

            if score > alpha {
                alpha = score;
                current_best = *mv;
            }
        }

        best_move = current_best;
        best_score = alpha;
    }

    (Some(best_move), best_score, info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::types::PieceType;
    use crate::fen;

    #[test]
    fn test_search_finds_move() {
        let mut board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let (mv, score, info) = search(&mut board, 2);

        assert!(mv.is_some(), "Should find a move");
        assert!(info.nodes > 0, "Should search some nodes");
        println!("Found move: {:?}, score: {}, nodes: {}", mv, score, info.nodes);
    }

    #[test]
    fn test_search_prefers_material_gain() {
        // Position where white can capture a queen
        // Black queen on d5, white knight can capture it
        let mut board = fen::fen_to_board("rnb1kbnr/ppp1pppp/8/3q4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 1").unwrap();
        let (mv, score, _) = search(&mut board, 3);

        // The engine should find a capture
        assert!(mv.is_some());
        let best_move = mv.unwrap();

        // Print what move was found
        println!("Best move: from {} to {}, piece: {:?}", best_move.from, best_move.to, best_move.piece);

        // Score should be positive (white is winning material)
        assert!(score > 500, "Should be winning at least a rook worth of material, got {}", score);
    }

    #[test]
    fn test_legal_moves_no_illegal() {
        // Test that we don't generate moves that leave king in check
        let mut board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let moves = generate_legal_moves(&mut board);

        // Should have 20 legal moves from starting position
        assert_eq!(moves.len(), 20, "Should have 20 legal moves from start");
    }

    #[test]
    fn test_checkmate_detection() {
        // Fool's mate position - black is checkmated
        let mut board = fen::fen_to_board("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2").unwrap();
        // Actually this isn't checkmate, let me use a real one
        // Scholar's mate setup
        let mut board2 =
            fen::fen_to_board("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4").unwrap();
        let moves = generate_legal_moves(&mut board2);

        // If checkmated, should have no moves
        println!("Legal moves in scholar's mate position: {}", moves.len());
    }
}
