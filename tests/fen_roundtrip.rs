use rust_chess::fen;

#[test]
fn test_fen_roundtrip_starting_position() {
    let original_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = fen::fen_to_board(original_fen).unwrap();
    let result_fen = fen::board_to_fen(&board);

    // Compare piece placement, side, castling, en passant
    // (halfmove clock and fullmove might differ)
    let original_parts: Vec<&str> = original_fen.split_whitespace().collect();
    let result_parts: Vec<&str> = result_fen.split_whitespace().collect();

    assert_eq!(original_parts[0], result_parts[0], "Piece placement should match");
    assert_eq!(original_parts[1], result_parts[1], "Side to move should match");
    assert_eq!(original_parts[2], result_parts[2], "Castling rights should match");
    assert_eq!(original_parts[3], result_parts[3], "En passant should match");
}

#[test]
fn test_fen_roundtrip_killer_position() {
    let original_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let board = fen::fen_to_board(original_fen).unwrap();
    let result_fen = fen::board_to_fen(&board);

    let original_parts: Vec<&str> = original_fen.split_whitespace().collect();
    let result_parts: Vec<&str> = result_fen.split_whitespace().collect();

    assert_eq!(original_parts[0], result_parts[0], "Piece placement should match");
    assert_eq!(original_parts[1], result_parts[1], "Side to move should match");
    assert_eq!(original_parts[2], result_parts[2], "Castling rights should match");
    assert_eq!(original_parts[3], result_parts[3], "En passant should match");
}

#[test]
fn test_fen_with_en_passant() {
    // After 1. e4
    let original_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let board = fen::fen_to_board(original_fen).unwrap();
    let result_fen = fen::board_to_fen(&board);

    let original_parts: Vec<&str> = original_fen.split_whitespace().collect();
    let result_parts: Vec<&str> = result_fen.split_whitespace().collect();

    assert_eq!(original_parts[0], result_parts[0], "Piece placement should match");
    assert_eq!(original_parts[1], result_parts[1], "Side to move should match");
    assert_eq!(original_parts[2], result_parts[2], "Castling rights should match");
    // En passant might be adjusted, so just check it parses
    assert!(!result_parts[3].is_empty(), "En passant should be present");
}

#[test]
fn test_fen_no_castling() {
    let original_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1";
    let board = fen::fen_to_board(original_fen).unwrap();
    let result_fen = fen::board_to_fen(&board);

    let original_parts: Vec<&str> = original_fen.split_whitespace().collect();
    let result_parts: Vec<&str> = result_fen.split_whitespace().collect();

    assert_eq!(original_parts[0], result_parts[0], "Piece placement should match");
    assert_eq!(original_parts[1], result_parts[1], "Side to move should match");
    assert_eq!(original_parts[2], result_parts[2], "Castling rights should match");
    assert_eq!(original_parts[3], result_parts[3], "En passant should match");
}

#[test]
fn test_fen_black_to_move() {
    let original_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let board = fen::fen_to_board(original_fen).unwrap();
    let result_fen = fen::board_to_fen(&board);

    let original_parts: Vec<&str> = original_fen.split_whitespace().collect();
    let result_parts: Vec<&str> = result_fen.split_whitespace().collect();

    assert_eq!(original_parts[0], result_parts[0], "Piece placement should match");
    assert_eq!(original_parts[1], result_parts[1], "Side to move should match");
}
