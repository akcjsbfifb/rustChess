use rustChess::board::Board;
use rustChess::movegen::perft;

#[test]
fn test_perft_depth_1() {
    let mut board = Board::new();
    let nodes = perft(&mut board, 1);
    assert_eq!(nodes, 20, "Perft 1 should be 20");
}

#[test]
fn test_perft_depth_2() {
    let mut board = Board::new();
    let nodes = perft(&mut board, 2);
    assert_eq!(nodes, 400, "Perft 2 should be 400");
}

#[test]
fn test_perft_depth_3() {
    let mut board = Board::new();
    let nodes = perft(&mut board, 3);
    assert_eq!(nodes, 8902, "Perft 3 should be 8902");
}

#[test]
fn test_perft_depth_4() {
    let mut board = Board::new();
    let nodes = perft(&mut board, 4);
    assert_eq!(nodes, 197281, "Perft 4 should be 197281");
}
