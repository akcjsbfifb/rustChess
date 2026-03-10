use crate::board::BLACK_OO;
use crate::board::BLACK_OOO;
use crate::board::Board;
use crate::board::UndoInfo;
use crate::board::WHITE_OO;
use crate::board::WHITE_OOO;
use crate::board::types::*;
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

    let mut undo_info: Vec<UndoInfo> = Vec::new();

    let mut side_to_move = Color::White;
    let mut white_king_index = E1;
    let mut black_king_index = E8;

    let mut en_passant_index: usize = 0;

    let v: Vec<&str> = fen.rsplit(['/', ' ']).rev().collect();
    let mut global_index = H8;
    for (i, s) in v.iter().enumerate() {
        let mut row_index = global_index - 7;
        for c in s.chars() {
            if i < 8 {
                if c.is_ascii_digit() {
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
                }
            } else if i == 10 && c != '-' {
                if c.is_ascii_digit() {
                    en_passant_index += c.to_digit(10).unwrap() as usize * 10;
                    en_passant_index -= 10;
                } else {
                    en_passant_index += 21;
                    en_passant_index += c.to_ascii_uppercase() as usize - 65;
                }
            }
        }
        if global_index >= 21 {
            global_index -= 10;
        }
    }

    if (21..=98).contains(&en_passant_index) {
        if side_to_move == Color::White {
            en_passant_index -= 10;
        } else {
            en_passant_index += 10;
        }
    }
    undo_info.push(UndoInfo {
        last_move: Move::new(0, 0, PieceType::None, PieceType::None, PieceType::None, 0),
        can_castle,
        en_passant_square: if (21..=98).contains(&en_passant_index) { Some(en_passant_index) } else { None },
        halfmove_clock: 0,
    });
    let result = Board::new(board, side_to_move, undo_info, can_castle, white_king_index, black_king_index);
    Ok(result)
}

pub fn board_to_fen(board: &Board) -> String {
    let mut fen = String::new();

    // 1. Piece placement
    for rank in (0..8).rev() {
        let mut empty_count = 0;
        for file in 0..8 {
            let sq = 21 + rank * 10 + file;
            let piece = board.squares[sq];

            if piece == 0 {
                empty_count += 1;
            } else {
                if empty_count > 0 {
                    fen.push_str(&empty_count.to_string());
                    empty_count = 0;
                }
                let c = match piece {
                    W_PAWN => 'P',
                    W_KNIGHT => 'N',
                    W_BISHOP => 'B',
                    W_ROOK => 'R',
                    W_QUEEN => 'Q',
                    W_KING => 'K',
                    B_PAWN => 'p',
                    B_KNIGHT => 'n',
                    B_BISHOP => 'b',
                    B_ROOK => 'r',
                    B_QUEEN => 'q',
                    B_KING => 'k',
                    _ => '?',
                };
                fen.push(c);
            }
        }
        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
        }
        if rank > 0 {
            fen.push('/');
        }
    }

    // 2. Side to move
    fen.push(' ');
    fen.push(match board.side_to_move {
        Color::White => 'w',
        Color::Black => 'b',
    });

    // 3. Castling rights
    fen.push(' ');
    let mut castling = String::new();
    if board.can_castle & WHITE_OO != 0 {
        castling.push('K');
    }
    if board.can_castle & WHITE_OOO != 0 {
        castling.push('Q');
    }
    if board.can_castle & BLACK_OO != 0 {
        castling.push('k');
    }
    if board.can_castle & BLACK_OOO != 0 {
        castling.push('q');
    }
    if castling.is_empty() {
        castling.push('-');
    }
    fen.push_str(&castling);

    // 4. En passant square
    fen.push(' ');
    if let Some(sq) = board.undo_info.last().and_then(|u| u.en_passant_square) {
        let file = (sq % 10) - 1;
        let rank = (sq / 10) - 2;
        let file_char = (b'a' + file as u8) as char;
        let rank_char = (b'1' + rank as u8) as char;
        fen.push(file_char);
        fen.push(rank_char);
    } else {
        fen.push('-');
    }

    // 5. Halfmove clock
    fen.push(' ');
    let halfmove = board.undo_info.last().map(|u| u.halfmove_clock).unwrap_or(0);
    fen.push_str(&halfmove.to_string());

    // 6. Fullmove number (default to 1 since Board doesn't store it)
    fen.push(' ');
    fen.push('1');

    fen
}
