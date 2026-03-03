use crate::board::Board;

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
    todo!()
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
