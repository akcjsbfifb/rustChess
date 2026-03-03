use rust_chess::board::Board;
use rust_chess::movegen::perft;

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

#[test]
fn test_perft_depth_5() {
    let mut board = Board::new();
    let nodes = perft(&mut board, 5);
    assert_eq!(nodes, 4865609, "Perft 5 should be 4865609");
}

// #[test]
// fn test_perft_depth_6() {
//     let mut board = Board::new();
//     let nodes = perft(&mut board, 6);
//     assert_eq!(nodes, 119060324, "Perft 6 should be 119060324");
// }
