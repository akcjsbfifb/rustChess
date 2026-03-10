use crate::board::types::*;
use crate::board::Board;
use crate::board::UndoInfo;
use crate::board::BLACK_OO;
use crate::board::BLACK_OOO;
use crate::board::WHITE_OO;
use crate::board::WHITE_OOO;
use crate::constants::*;
/// Parsea un string FEN y devuelve un Board.
///
/// # Arguments
/// * `fen` - String en formato FEN estándar
///
/// # Returns
/// * `Result<Board, String>` - Board inicializado o error si el FEN es inválido
///
/// # Formato FEN
/// 6 campos separados por espacio:
/// 1. Piece placement: r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R
/// 2. Active color: w o b
/// 3. Castling rights: KQkq, KQ, K, Q, o -
/// 4. En passant square: e3 o -
/// 5. Halfmove clock: número
/// 6. Fullmove number: número
pub fn fen_to_board(fen: &str) -> Result<Board, String> {
    let mut board: [u8; BOARD_SIZE] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    let mut can_castle: u8 = 0b1111;
    
    let undo_info: Vec<UndoInfo> = Vec::new();

    let mut side_to_move = Color::White;
    let mut white_king_index = E1;
    let mut black_king_index = E8;

    let mut info_for_undo: UndoInfo;

    let v: Vec<&str> = fen.rsplit(|c| c == '/' || c == ' ').rev().collect();
    let mut global_index = H8;
    for (i, s) in v.iter().enumerate() {
        println!("{}: {}", i, s);

        let mut row_index = global_index - 7;
        for c in s.chars() {
            if i < 8 {
                if c.is_digit(10) {
                    row_index += c.to_digit(10).unwrap() as usize;
                } else {
                    match c {
                        'P' => board[row_index] = W_PAWN,
                        'N' => board[row_index] = W_KNIGHT,
                        'B' => board[row_index] = W_BISHOP,
                        'R' => board[row_index] = W_ROOK,
                        'Q' => board[row_index] = W_QUEEN,
                        'K' => {
                            board[row_index] = W_KING;
                            white_king_index = row_index;
                        }
                        'p' => board[row_index] = B_PAWN,
                        'n' => board[row_index] = B_KNIGHT,
                        'b' => board[row_index] = B_BISHOP,
                        'r' => board[row_index] = B_ROOK,
                        'q' => board[row_index] = B_QUEEN,
                        'k' => {
                            board[row_index] = B_KING;
                            black_king_index = row_index;
                        }
                        _ => return Err(format!("Carácter inválido en FEN: '{}'", c)),
                    }

                    println!("row_index char:: {}\n", row_index);
                    row_index += 1;
                }
            } else if i == 8 {
                match c {
                    'w' => side_to_move = Color::White,
                    'b' => side_to_move = Color::Black,
                    _ => return Err(format!("Carácter inválido en FEN: '{}'", c)),
                }
            } else if i == 9 {
                if c == '-' {
                    can_castle = 0b0000;
                } else {
                    match c {
                        'K' => can_castle |= WHITE_OO,
                        'Q' => can_castle |= WHITE_OOO,
                        'k' => can_castle |= BLACK_OO,
                        'q' => can_castle |= BLACK_OOO,
                        _ => return Err(format!("Carácter inválido en FEN: '{}'", c)),
                    }
                    println!("can_castle:: {}", can_castle);
                }
            }
        }
        if global_index >= 21 {
            global_index -= 10;
        }
    }
    let result = Board::new(board, side_to_move, undo_info, can_castle, white_king_index, black_king_index);
    Ok(result)
}

/// Convierte un Board a string FEN.
///
/// # Arguments
/// * `board` - Referencia al Board
///
/// # Returns
/// * String en formato FEN
pub fn board_to_fen(board: &Board) -> String {
    todo!()
}
