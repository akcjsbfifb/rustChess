use rust_chess::board::Board;
use rust_chess::fen;
use rust_chess::movegen::perft;

#[test]
fn test_perft_depth_1() {
    let mut board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 1);
    assert_eq!(nodes, 20, "Perft 1 should be 20");
}

#[test]
fn test_perft_depth_2() {
    let mut board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 2);
    assert_eq!(nodes, 400, "Perft 2 should be 400");
}

#[test]
fn test_perft_depth_3() {
    let mut board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 3);
    assert_eq!(nodes, 8902, "Perft 3 should be 8902");
}

#[test]
fn test_perft_depth_4() {
    let mut board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 4);
    assert_eq!(nodes, 197281, "Perft 4 should be 197281");
}

#[test]
#[ignore = "takes too long for regular testing"]
fn test_perft_depth_5() {
    let mut board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 5);
    assert_eq!(nodes, 4865609, "Perft 5 should be 4865609");
}

#[test]
#[ignore = "takes too long for regular testing"]
fn test_perft_depth_6() {
    let mut board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 6);
    assert_eq!(nodes, 119060324, "Perft 6 should be 119060324");
}

#[test]
fn test_perft_killer_position_depth_1() {
    let mut board = fen::fen_to_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 1);
    assert_eq!(nodes, 48, "Perft 1 should be 48");
}

#[test]
fn test_perft_killer_position_depth_2() {
    let mut board = fen::fen_to_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 2);
    assert_eq!(nodes, 2039, "Perft 2 should be 2039");
}

#[test]
fn test_perft_killer_position_depth_3() {
    let mut board = fen::fen_to_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 3);
    assert_eq!(nodes, 97862, "Perft 3 should be 97862");
}

#[test]
fn test_perft_killer_position_depth_4() {
    let mut board = fen::fen_to_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 4);
    assert_eq!(nodes, 4085603, "Perft 4 should be 4085603");
}

#[test]
#[ignore = "takes too long for regular testing"]
fn test_perft_killer_position_depth_5() {
    let mut board = fen::fen_to_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 5);
    assert_eq!(nodes, 193690690, "Perft 5 should be 193690690");
}

#[test]
#[ignore = "takes too long for regular testing"]
fn test_perft_killer_position_depth_6() {
    let mut board = fen::fen_to_board("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let nodes = perft(&mut board, 6);
    assert_eq!(nodes, 8031647685, "Perft 6 should be 8031647685");
}
