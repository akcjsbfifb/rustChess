// Estos tests prueban la lógica de Board, Move y MoveGen.
use rust_chess::board::types::{Color, Move, PieceType};
use rust_chess::board::{Board, BLACK_OO, BLACK_OOO, WHITE_OO, WHITE_OOO};
use rust_chess::constants::*;
use rust_chess::movegen::generate_moves;

#[test]
fn test_make_unmake_simple_move() {
    let mut board = Board::new();
    let initial_squares = board.squares.clone();

    // Movimiento de peón e2-e4 (casillas 35 a 55 en 10x12)
    let m = Move::new(35, 55, PieceType::Pawn, PieceType::None, PieceType::None, Move::FLAG_DOUBLE_PAWN);

    board.make_move(m);
    assert_ne!(board.squares, initial_squares, "El tablero debería haber cambiado tras make_move");
    assert_eq!(board.side_to_move, Color::Black, "Debería ser el turno de las negras");

    board.unmake_move();
    assert_eq!(board.squares, initial_squares, "El tablero NO se restauró perfectamente tras unmake_move");
    assert_eq!(board.side_to_move, Color::White, "Debería volver a ser el turno de las blancas");
}

#[test]
fn test_capture_restoration() {
    let mut board = Board::new();
    // Simulamos una pieza negra en e4 para capturar
    board.squares[55] = 0x82; // Caballo negro
    let initial_squares = board.squares.clone();

    let m = Move::new(35, 55, PieceType::Pawn, PieceType::Knight, PieceType::None, Move::FLAG_NONE);

    board.make_move(m);
    assert_eq!(board.get_piece_type(55), PieceType::Pawn, "El peón debería estar en la casilla de captura");

    board.unmake_move();
    assert_eq!(board.squares, initial_squares, "La pieza capturada no se restauró correctamente");
}

#[test]
fn test_white_kingside_castle_consistency() {
    let mut board = Board::new();
    // Limpiamos f1 y g1 para permitir enroque
    board.squares[26] = EMPTY;
    board.squares[27] = EMPTY;
    let initial_squares = board.squares.clone();

    let m = Move::new(25, 27, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_KING);

    board.make_move(m);
    assert_eq!(board.squares[27], 0x06, "El rey debería estar en g1");
    assert_eq!(board.squares[26], 0x04, "La torre debería estar en f1");

    board.unmake_move();
    assert_eq!(board.squares, initial_squares, "El tablero no se restauró correctamente tras enroque");
}

#[test]
fn test_white_queenside_castle_consistency() {
    let mut board = Board::new();
    board.squares[22] = EMPTY;
    board.squares[23] = EMPTY;
    board.squares[24] = EMPTY;
    let initial_squares = board.squares.clone();

    let m = Move::new(25, 23, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_QUEEN);

    board.make_move(m);
    assert_eq!(board.squares[23], 0x06, "El rey debería estar en c1");
    assert_eq!(board.squares[24], 0x04, "La torre debería estar en d1");

    board.unmake_move();
    assert_eq!(board.squares, initial_squares, "El tablero no se restauró correctamente tras enroque");
}

#[test]
fn test_black_kingside_castle_consistency() {
    let mut board = Board::new();
    board.side_to_move = Color::Black;
    board.squares[96] = EMPTY;
    board.squares[97] = EMPTY;
    let initial_squares = board.squares.clone();

    let m = Move::new(95, 97, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_KING);

    board.make_move(m);
    assert_eq!(board.squares[97], 0x86, "El rey debería estar en g8");
    assert_eq!(board.squares[96], 0x84, "La torre debería estar en f8");

    board.unmake_move();
    assert_eq!(board.squares, initial_squares, "El tablero no se restauró correctamente tras enroque");
}

#[test]
fn test_black_queenside_castle_consistency() {
    let mut board = Board::new();
    board.side_to_move = Color::Black;
    board.squares[93] = EMPTY;
    board.squares[94] = EMPTY;
    let initial_squares = board.squares.clone();

    let m = Move::new(95, 93, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_QUEEN);

    board.make_move(m);
    assert_eq!(board.squares[93], 0x86, "El rey debería estar en c8");
    assert_eq!(board.squares[94], 0x84, "La torre debería estar en d8");

    board.unmake_move();
    assert_eq!(board.squares, initial_squares, "El tablero no se restauró correctamente tras enroque");
}

#[test]
fn test_en_passant_removal() {
    let mut board = Board::new();
    // Preparamos peón blanco en quinta y peón negro al lado que acaba de hacer double move
    board.squares[65] = 0x01; // Peón blanco en e5
    board.squares[66] = 0x81; // Peón negro en f5

    // Simulamos que el último movimiento fue el doble avance del negro a f5
    board.undo_info.push(rust_chess::board::UndoInfo {
        last_move: Move::new(86, 66, PieceType::Pawn, PieceType::None, PieceType::None, Move::FLAG_DOUBLE_PAWN),
        can_castle: 0,
        en_passant_square: Some(66),
        halfmove_clock: 0,
    });

    let m = Move::new(65, 76, PieceType::Pawn, PieceType::Pawn, PieceType::None, Move::FLAG_EN_PASSANT);

    board.make_move(m);
    assert_eq!(board.squares[66], EMPTY, "El peón capturado al paso no fue eliminado");

    board.unmake_move();
    assert_eq!(board.squares[66], 0x81, "El peón capturado al paso no fue restaurado");
}

#[test]
fn test_promotion_unmake_logic() {
    let mut board = Board::new();
    // Ponemos peón blanco en 7ma y pieza negra en 8va para capturar y promocionar
    board.squares[84] = 0x01; // Peón blanco en e7
    board.squares[95] = 0x85; // Dama negra en e8
    let initial_squares = board.squares.clone();

    let m = Move::new(84, 95, PieceType::Pawn, PieceType::Queen, PieceType::Queen, Move::FLAG_PROMOTION);

    board.make_move(m);
    assert_eq!(board.squares[95], 0x05, "Debería haber una Dama Blanca en e8");

    board.unmake_move();
    assert_eq!(board.squares[84], 0x01, "Debería volver a ser un Peón Blanco en e7");
    assert_eq!(board.squares[95], 0x85, "Debería volver a estar la Dama Negra en e8");
    assert_eq!(board.squares, initial_squares, "El estado total tras deshacer promoción es incorrecto");
}

#[test]
fn test_first_move_no_panic() {
    let board = Board::new();
    let _moves = generate_moves(&board);
}

mod castling_tests {
    use super::*;

    #[test]
    fn test_white_kingside_castle_move_and_restore() {
        let mut board = Board::new();
        assert_eq!(board.can_castle, 0b1111);

        board.squares[F1] = EMPTY;
        board.squares[G1] = EMPTY;

        let m = Move::new(E1, G1, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_KING);
        board.make_move(m);

        assert_eq!(board.squares[G1], W_KING);
        assert_eq!(board.squares[F1], W_ROOK);
        assert_eq!(board.can_castle & WHITE_OO, 0, "White OO should be removed");
        assert_eq!(board.can_castle & WHITE_OOO, 0, "White OO should be removed");

        board.unmake_move();

        assert_eq!(board.squares[E1], W_KING);
        assert_eq!(board.squares[H1], W_ROOK);
        assert_eq!(board.squares[F1], EMPTY);
        assert_eq!(board.can_castle, 0b1111, "Castling rights should be restored");
    }

    #[test]
    fn test_white_queenside_castle_move_and_restore() {
        let mut board = Board::new();
        assert_eq!(board.can_castle, 0b1111);

        board.squares[B1] = EMPTY;
        board.squares[C1] = EMPTY;
        board.squares[D1] = EMPTY;

        let m = Move::new(E1, C1, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_QUEEN);
        board.make_move(m);

        assert_eq!(board.squares[C1], W_KING);
        assert_eq!(board.squares[D1], W_ROOK);
        assert_eq!(board.can_castle & WHITE_OO, 0, "White OOO should be removed");
        assert_eq!(board.can_castle & WHITE_OOO, 0, "White OOO should be removed");

        board.unmake_move();

        assert_eq!(board.squares[E1], W_KING);
        assert_eq!(board.squares[A1], W_ROOK);
        assert_eq!(board.can_castle, 0b1111);
    }

    #[test]
    fn test_black_kingside_castle_move_and_restore() {
        let mut board = Board::new();
        board.side_to_move = Color::Black;
        assert_eq!(board.can_castle, 0b1111);

        board.squares[F8] = EMPTY;
        board.squares[G8] = EMPTY;

        let m = Move::new(E8, G8, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_KING);
        board.make_move(m);

        assert_eq!(board.squares[G8], B_KING);
        assert_eq!(board.squares[F8], B_ROOK);
        assert_eq!(board.can_castle & BLACK_OO, 0, "Black OO should be removed");
        assert_eq!(board.can_castle & BLACK_OOO, 0, "Black OOO should be removed");

        board.unmake_move();

        assert_eq!(board.squares[E8], B_KING);
        assert_eq!(board.squares[H8], B_ROOK);
        assert_eq!(board.can_castle, 0b1111);
    }

    #[test]
    fn test_black_queenside_castle_move_and_restore() {
        let mut board = Board::new();
        board.side_to_move = Color::Black;
        assert_eq!(board.can_castle, 0b1111);

        board.squares[B8] = EMPTY;
        board.squares[C8] = EMPTY;
        board.squares[D8] = EMPTY;

        let m = Move::new(E8, C8, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_QUEEN);
        board.make_move(m);

        assert_eq!(board.squares[C8], B_KING);
        assert_eq!(board.squares[D8], B_ROOK);
        assert_eq!(board.can_castle & BLACK_OO, 0, "Black OO should be removed");
        assert_eq!(board.can_castle & BLACK_OOO, 0, "Black OOO should be removed");

        board.unmake_move();

        assert_eq!(board.squares[E8], B_KING);
        assert_eq!(board.squares[A8], B_ROOK);
        assert_eq!(board.can_castle, 0b1111);
    }

    #[test]
    fn test_king_move_removes_all_castling_rights_for_color() {
        let mut board = Board::new();

        let m = Move::new(E1, E2, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_NONE);
        board.make_move(m);

        assert_eq!(board.can_castle & WHITE_OO, 0, "White OO should be removed when king moves");
        assert_eq!(board.can_castle & WHITE_OOO, 0, "White OOO should be removed when king moves");
        assert_eq!(board.can_castle & (BLACK_OO | BLACK_OOO), BLACK_OO | BLACK_OOO, "Black rights should remain");

        board.unmake_move();
        assert_eq!(board.can_castle, 0b1111);
    }

    #[test]
    fn test_rook_move_from_h1_removes_white_oo() {
        let mut board = Board::new();

        let m = Move::new(H1, A1, PieceType::Rook, PieceType::None, PieceType::None, Move::FLAG_NONE);
        board.make_move(m);

        assert_eq!(board.can_castle & WHITE_OO, 0, "White OO should be removed when rook from H1 moves");
        assert_eq!(board.can_castle & WHITE_OOO, WHITE_OOO, "White OOO should remain");

        board.unmake_move();
        assert_eq!(board.can_castle, 0b1111);
    }

    #[test]
    fn test_rook_move_from_a1_removes_white_ooo() {
        let mut board = Board::new();

        let m = Move::new(A1, H1, PieceType::Rook, PieceType::None, PieceType::None, Move::FLAG_NONE);
        board.make_move(m);

        assert_eq!(board.can_castle & WHITE_OOO, 0, "White OOO should be removed when rook from A1 moves");
        assert_eq!(board.can_castle & WHITE_OO, WHITE_OO, "White OO should remain");

        board.unmake_move();
        assert_eq!(board.can_castle, 0b1111);
    }

    #[test]
    fn test_rook_capture_on_h8_removes_black_oo() {
        let mut board = Board::new();
        board.side_to_move = Color::White;
        assert_eq!(board.can_castle, 0b1111);

        let m = Move::new(E2, H8, PieceType::Pawn, PieceType::Rook, PieceType::None, Move::FLAG_NONE);
        board.make_move(m);

        assert_eq!(board.can_castle & BLACK_OO, 0, "Black OO should be removed when rook on H8 is captured");
        assert_eq!(board.can_castle & BLACK_OOO, BLACK_OOO, "Black OOO should remain");

        board.unmake_move();
        assert_eq!(board.can_castle, 0b1111);
    }

    #[test]
    fn test_rook_capture_on_a8_removes_black_ooo() {
        let mut board = Board::new();
        board.side_to_move = Color::White;
        assert_eq!(board.can_castle, 0b1111);

        let m = Move::new(E2, A8, PieceType::Pawn, PieceType::Rook, PieceType::None, Move::FLAG_NONE);
        board.make_move(m);

        assert_eq!(board.can_castle & BLACK_OOO, 0, "Black OOO should be removed when rook on A8 is captured");
        assert_eq!(board.can_castle & BLACK_OO, BLACK_OO, "Black OO should remain");

        board.unmake_move();
        assert_eq!(board.can_castle, 0b1111);
    }

    #[test]
    fn test_both_rooks_captured_removes_all_black_castling() {
        let mut board = Board::new();
        board.side_to_move = Color::White;
        assert_eq!(board.can_castle, 0b1111);

        let m1 = Move::new(E2, H8, PieceType::Pawn, PieceType::Rook, PieceType::None, Move::FLAG_NONE);
        board.make_move(m1);
        assert_eq!(board.can_castle & BLACK_OO, 0, "First rook captured");

        board.side_to_move = Color::White;

        let m2 = Move::new(E3, A8, PieceType::Pawn, PieceType::Rook, PieceType::None, Move::FLAG_NONE);
        board.make_move(m2);

        assert_eq!(board.can_castle & BLACK_OO, 0);
        assert_eq!(board.can_castle & BLACK_OOO, 0, "All black castling rights should be removed");

        board.unmake_move();
        board.unmake_move();
        assert_eq!(board.can_castle, 0b1111);
    }
}

mod king_index_tests {
    use super::*;

    #[test]
    fn test_white_king_index_initial() {
        let board = Board::new();
        assert_eq!(board.white_king_index, E1);
        assert_eq!(board.black_king_index, E8);
    }

    #[test]
    fn test_white_king_moves_updates_index() {
        let mut board = Board::new();

        let m = Move::new(E1, E2, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_NONE);
        board.make_move(m);

        assert_eq!(board.white_king_index, E2);

        board.unmake_move();
        assert_eq!(board.white_king_index, E1);
    }

    #[test]
    fn test_black_king_moves_updates_index() {
        let mut board = Board::new();
        board.side_to_move = Color::Black;

        let m = Move::new(E8, E7, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_NONE);
        board.make_move(m);

        assert_eq!(board.black_king_index, E7);

        board.unmake_move();
        assert_eq!(board.black_king_index, E8);
    }

    #[test]
    fn test_white_castle_kingside_updates_king_index() {
        let mut board = Board::new();
        board.squares[F1] = EMPTY;
        board.squares[G1] = EMPTY;

        let m = Move::new(E1, G1, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_KING);
        board.make_move(m);

        assert_eq!(board.white_king_index, G1);

        board.unmake_move();
        assert_eq!(board.white_king_index, E1);
    }

    #[test]
    fn test_white_castle_queenside_updates_king_index() {
        let mut board = Board::new();
        board.squares[B1] = EMPTY;
        board.squares[C1] = EMPTY;
        board.squares[D1] = EMPTY;

        let m = Move::new(E1, C1, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_QUEEN);
        board.make_move(m);

        assert_eq!(board.white_king_index, C1);

        board.unmake_move();
        assert_eq!(board.white_king_index, E1);
    }

    #[test]
    fn test_black_castle_kingside_updates_king_index() {
        let mut board = Board::new();
        board.side_to_move = Color::Black;
        board.squares[F8] = EMPTY;
        board.squares[G8] = EMPTY;

        let m = Move::new(E8, G8, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_KING);
        board.make_move(m);

        assert_eq!(board.black_king_index, G8);

        board.unmake_move();
        assert_eq!(board.black_king_index, E8);
    }

    #[test]
    fn test_black_castle_queenside_updates_king_index() {
        let mut board = Board::new();
        board.side_to_move = Color::Black;
        board.squares[B8] = EMPTY;
        board.squares[C8] = EMPTY;
        board.squares[D8] = EMPTY;

        let m = Move::new(E8, C8, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_QUEEN);
        board.make_move(m);

        assert_eq!(board.black_king_index, C8);

        board.unmake_move();
        assert_eq!(board.black_king_index, E8);
    }
}
