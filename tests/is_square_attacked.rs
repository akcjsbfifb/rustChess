use rustChess::{
    board::{
        types::{Color, PieceType},
        Board,
    },
    constants::*,
    movegen::is_square_attacked,
};

fn setup_board(pieces: &[(usize, u8)]) -> Board {
    let mut board = Board::new();
    for i in 0..BOARD_SIZE {
        if board.squares[i] != OFF_BOARD {
            board.squares[i] = EMPTY;
        }
    }
    for &(square, piece_code) in pieces {
        board.squares[square] = piece_code;
    }
    board
}

mod pawn_attacks {
    use super::*;

    #[test]
    fn white_pawn_attacks_diagonal_left() {
        let board = setup_board(&[(55, W_PAWN)]);
        assert!(is_square_attacked(&board, 55 + 9, Color::White));
    }

    #[test]
    fn white_pawn_attacks_diagonal_right() {
        let board = setup_board(&[(55, W_PAWN)]);
        assert!(is_square_attacked(&board, 55 + 11, Color::White));
    }

    #[test]
    fn white_pawn_does_not_attack_forward() {
        let board = setup_board(&[(55, W_PAWN)]);
        assert!(!is_square_attacked(&board, 55 + 10, Color::White));
    }

    #[test]
    fn black_pawn_attacks_diagonal_left() {
        let board = setup_board(&[(65, B_PAWN)]);
        assert!(is_square_attacked(&board, 65 - 9, Color::Black));
    }

    #[test]
    fn black_pawn_attacks_diagonal_right() {
        let board = setup_board(&[(65, B_PAWN)]);
        assert!(is_square_attacked(&board, 65 - 11, Color::Black));
    }

    #[test]
    fn black_pawn_does_not_attack_forward() {
        let board = setup_board(&[(65, B_PAWN)]);
        assert!(!is_square_attacked(&board, 65 - 10, Color::Black));
    }
}

mod knight_attacks {
    use super::*;

    #[test]
    fn knight_attacks_all_8_squares() {
        let board = setup_board(&[(55, W_KNIGHT)]);
        assert!(is_square_attacked(&board, 55 + 21, Color::White));
        assert!(is_square_attacked(&board, 55 + 19, Color::White));
        assert!(is_square_attacked(&board, 55 + 12, Color::White));
        assert!(is_square_attacked(&board, 55 + 8, Color::White));
        assert!(is_square_attacked(&board, 55 - 21, Color::White));
        assert!(is_square_attacked(&board, 55 - 19, Color::White));
        assert!(is_square_attacked(&board, 55 - 12, Color::White));
        assert!(is_square_attacked(&board, 55 - 8, Color::White));
    }

    #[test]
    fn knight_does_not_attack_adjacent_squares() {
        let board = setup_board(&[(55, W_KNIGHT)]);
        assert!(!is_square_attacked(&board, 55 + 10, Color::White));
        assert!(!is_square_attacked(&board, 55 + 1, Color::White));
    }
}

mod bishop_attacks {
    use super::*;

    #[test]
    fn bishop_attacks_diagonals() {
        let board = setup_board(&[(55, W_BISHOP)]);
        assert!(is_square_attacked(&board, 55 + 11, Color::White));
        assert!(is_square_attacked(&board, 55 + 9, Color::White));
        assert!(is_square_attacked(&board, 55 - 11, Color::White));
        assert!(is_square_attacked(&board, 55 - 9, Color::White));
    }

    #[test]
    fn bishop_does_not_attack_orthogonally() {
        let board = setup_board(&[(55, W_BISHOP)]);
        assert!(!is_square_attacked(&board, 55 + 10, Color::White));
        assert!(!is_square_attacked(&board, 55 - 10, Color::White));
        assert!(!is_square_attacked(&board, 55 + 1, Color::White));
        assert!(!is_square_attacked(&board, 55 - 1, Color::White));
    }

    #[test]
    fn bishop_attack_blocked_by_piece_same_color() {
        let board = setup_board(&[(C3, W_BISHOP), (E5, W_PAWN)]);
        assert!(!is_square_attacked(&board, G7, Color::White));
    }

    #[test]
    fn bishop_attack_blocked_by_piece_dif_color() {
        let board = setup_board(&[(C3, W_BISHOP), (E5, B_PAWN)]);
        assert!(!is_square_attacked(&board, G7, Color::White));
    }
}

mod rook_attacks {
    use super::*;

    #[test]
    fn rook_attacks_orthogonals() {
        let board = setup_board(&[(55, W_ROOK)]);
        assert!(is_square_attacked(&board, 55 + 10, Color::White));
        assert!(is_square_attacked(&board, 55 - 10, Color::White));
        assert!(is_square_attacked(&board, 55 + 1, Color::White));
        assert!(is_square_attacked(&board, 55 - 1, Color::White));
    }

    #[test]
    fn rook_does_not_attack_diagonally() {
        let board = setup_board(&[(55, W_ROOK)]);
        assert!(!is_square_attacked(&board, 55 + 11, Color::White));
        assert!(!is_square_attacked(&board, 55 + 9, Color::White));
    }

    #[test]
    fn rook_attack_blocked_by_piece() {
        let board = setup_board(&[(55, W_ROOK), (55 + 20, W_PAWN)]);
        assert!(!is_square_attacked(&board, 55 + 30, Color::White));
    }
}

mod queen_attacks {
    use super::*;

    #[test]
    fn queen_attacks_all_directions() {
        let board = setup_board(&[(55, W_QUEEN)]);
        assert!(is_square_attacked(&board, 55 + 10, Color::White));
        assert!(is_square_attacked(&board, 55 - 10, Color::White));
        assert!(is_square_attacked(&board, 55 + 1, Color::White));
        assert!(is_square_attacked(&board, 55 - 1, Color::White));
        assert!(is_square_attacked(&board, 55 + 11, Color::White));
        assert!(is_square_attacked(&board, 55 + 9, Color::White));
        assert!(is_square_attacked(&board, 55 - 11, Color::White));
        assert!(is_square_attacked(&board, 55 - 9, Color::White));
    }
}

mod king_attacks {
    use super::*;

    #[test]
    fn king_attacks_adjacent_squares() {
        let board = setup_board(&[(55, W_KING)]);
        assert!(is_square_attacked(&board, 55 + 10, Color::White));
        assert!(is_square_attacked(&board, 55 - 10, Color::White));
        assert!(is_square_attacked(&board, 55 + 1, Color::White));
        assert!(is_square_attacked(&board, 55 - 1, Color::White));
        assert!(is_square_attacked(&board, 55 + 11, Color::White));
        assert!(is_square_attacked(&board, 55 + 9, Color::White));
        assert!(is_square_attacked(&board, 55 - 11, Color::White));
        assert!(is_square_attacked(&board, 55 - 9, Color::White));
    }

    #[test]
    fn king_does_not_attack_two_squares_away() {
        let board = setup_board(&[(55, W_KING)]);
        assert!(!is_square_attacked(&board, 55 + 20, Color::White));
        assert!(!is_square_attacked(&board, 55 + 2, Color::White));
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn empty_square_not_attacked() {
        let board = setup_board(&[]);
        assert!(!is_square_attacked(&board, 55, Color::White));
    }

    #[test]
    fn off_board_returns_false() {
        let board = setup_board(&[(21, W_KNIGHT)]);
        assert!(!is_square_attacked(&board, 0, Color::White));
    }
}
