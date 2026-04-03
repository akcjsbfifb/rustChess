pub mod display;
pub mod types;

use crate::constants::*;
use types::*;

const INITIAL_POSITION: [u8; BOARD_SIZE] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 0  - borde
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 1  - borde
    0xFF, 0x04, 0x02, 0x03, 0x05, 0x06, 0x03, 0x02, 0x04, 0xFF, // 2  - rank 1 (blancas)
    0xFF, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0xFF, // 3  - rank 2
    0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 4  - rank 3
    0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 5  - rank 4
    0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 6  - rank 5
    0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 7  - rank 6
    0xFF, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0xFF, // 8  - rank 7
    0xFF, 0x84, 0x82, 0x83, 0x85, 0x86, 0x83, 0x82, 0x84, 0xFF, // 9  - rank 8 (negras)
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 10 - borde
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 11 - borde
];

// const INITIAL_POSITION: [u8; BOARD_SIZE] = [
//     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 0  - borde
//     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 1  - borde
//     0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 4  - rank 3
//     0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 4  - rank 3
//     0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 4  - rank 3
//     0xFF, 0x00, 0x00, 0x00, 0x06, 0x02, 0x00, 0x00, 0x00, 0xFF, // 5  - rank 4
//     0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 6  - rank 5
//     0xFF, 0x00, 0x01, 0x00, 0x83, 0x00, 0x81, 0x00, 0x00, 0xFF, // 7  - rank 6
//     0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 5  - rank 4
//     0xFF, 0x00, 0x00, 0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // 5  - rank 4
//     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 10 - borde
//     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // 11 - borde
// ];

pub const WHITE_OO: u8 = 0b0001; // Corto blancas
pub const WHITE_OOO: u8 = 0b0010; // Largo blancas
pub const BLACK_OO: u8 = 0b0100; // Corto negras
pub const BLACK_OOO: u8 = 0b1000; // Largo negras

#[derive(Clone)]
pub struct UndoInfo {
    pub last_move: Move,
    pub can_castle: u8,
    pub en_passant_square: Option<usize>,
    pub halfmove_clock: u16,
}

pub struct Board {
    pub squares: [u8; BOARD_SIZE],
    pub side_to_move: Color,
    pub undo_info: Vec<UndoInfo>,
    pub can_castle: u8, // blancas rey, blanca dana, negras rey, negras dana
    pub white_king_index: usize,
    pub black_king_index: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::new(INITIAL_POSITION, Color::White, Vec::new(), 0b1111, E1, E8)
    }
}

impl Board {
    pub fn new(
        squares: [u8; BOARD_SIZE],
        side_to_move: Color,
        undo_info: Vec<UndoInfo>,
        can_castle: u8,
        white_king_index: usize,
        black_king_index: usize,
    ) -> Self {
        Board {
            squares,
            side_to_move,
            undo_info,
            can_castle, // Ambos lados pueden enrocarse
            white_king_index,
            black_king_index,
        }
    }

    pub fn get_piece_type(&self, index: usize) -> PieceType {
        let p = self.squares[index];
        if p == OFF_BOARD || p == EMPTY {
            return PieceType::None;
        }
        match p & TYPE_MASK {
            1 => PieceType::Pawn,
            2 => PieceType::Knight,
            3 => PieceType::Bishop,
            4 => PieceType::Rook,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => PieceType::None,
        }
    }

    pub fn is_color(&self, index: usize, color: Color) -> bool {
        let piece = self.squares[index];
        if piece == EMPTY || piece == OFF_BOARD {
            return false;
        }
        match color {
            Color::White => (piece & COLOR_BIT) == 0,
            Color::Black => (piece & COLOR_BIT) != 0,
        }
    }

    pub fn make_move(&mut self, _mv: Move) {
        let mut last_move = UndoInfo {
            last_move: _mv,
            can_castle: self.can_castle,
            en_passant_square: None,
            halfmove_clock: 0, // TODO: Update halfmove clock
        };

        self.squares[_mv.to] = self.squares[_mv.from];
        match _mv.flags {
            Move::FLAG_DOUBLE_PAWN => {
                // Set en passant square
                last_move.en_passant_square = Some(_mv.to);
            }
            Move::FLAG_EN_PASSANT => {
                // Remove captured pawn
                if self.side_to_move == Color::White {
                    self.squares[_mv.to - 10] = EMPTY; // Captura al paso para blancas
                } else {
                    self.squares[_mv.to + 10] = EMPTY; // Captura al paso para negras
                }
            }
            Move::FLAG_CASTLE_KING | Move::FLAG_CASTLE_QUEEN => match self.side_to_move {
                Color::White => {
                    if _mv.flags == Move::FLAG_CASTLE_KING {
                        self.squares[H1] = EMPTY;
                        self.squares[F1] = W_ROOK;
                    } else {
                        self.squares[A1] = EMPTY;
                        self.squares[D1] = W_ROOK;
                    }
                }
                Color::Black => {
                    if _mv.flags == Move::FLAG_CASTLE_KING {
                        self.squares[H8] = EMPTY;
                        self.squares[F8] = B_ROOK;
                    } else {
                        self.squares[A8] = EMPTY;
                        self.squares[D8] = B_ROOK;
                    }
                }
            },
            Move::FLAG_PROMOTION => {
                // Promote pawn to chosen piece
                self.squares[_mv.to] = match _mv.promotion {
                    PieceType::Queen => {
                        if self.side_to_move == Color::White {
                            W_QUEEN
                        } else {
                            B_QUEEN
                        }
                    }
                    PieceType::Rook => {
                        if self.side_to_move == Color::White {
                            W_ROOK
                        } else {
                            B_ROOK
                        }
                    }
                    PieceType::Bishop => {
                        if self.side_to_move == Color::White {
                            W_BISHOP
                        } else {
                            B_BISHOP
                        }
                    }
                    PieceType::Knight => {
                        if self.side_to_move == Color::White {
                            W_KNIGHT
                        } else {
                            B_KNIGHT
                        }
                    }
                    _ => self.squares[_mv.from],
                };
            }
            _ => {}
        }
        if _mv.piece == PieceType::King {
            // Update castling rights if king moves
            // change king position in kings_index
            match self.side_to_move {
                Color::White => {
                    self.can_castle &= 0b1100;
                    self.white_king_index = _mv.to;
                } // Quitar derechos de enroque para blancas
                Color::Black => {
                    self.can_castle &= 0b0011;
                    self.black_king_index = _mv.to;
                } // Quitar derechos de enroque para negras
            }
        }
        if _mv.piece == PieceType::Rook {
            // Update castling rights if rook moves
            match self.side_to_move {
                Color::White => {
                    if _mv.from == A1 {
                        self.can_castle &= 0b1101;
                    } else if _mv.from == H1 {
                        self.can_castle &= 0b1110;
                    }
                }
                Color::Black => {
                    if _mv.from == A8 {
                        self.can_castle &= 0b0111;
                    } else if _mv.from == H8 {
                        self.can_castle &= 0b1011;
                    }
                }
            }
        }
        if _mv.captured == PieceType::Rook {
            // Update castling rights if rook is captured
            match self.side_to_move.opponent() {
                Color::White => {
                    if _mv.to == A1 {
                        self.can_castle &= 0b1101;
                    } else if _mv.to == H1 {
                        self.can_castle &= 0b1110;
                    }
                }
                Color::Black => {
                    if _mv.to == A8 {
                        self.can_castle &= 0b0111;
                    } else if _mv.to == H8 {
                        self.can_castle &= 0b1011;
                    }
                }
            }
        }

        self.squares[_mv.from] = EMPTY;
        self.side_to_move = self.side_to_move.opponent();
        self.undo_info.push(last_move);
    }

    pub fn unmake_move(&mut self) {
        // TODO: Implement unmake_move
        // 1. Restore the piece to its original square
        // 2. Restore captured piece if any
        // 3. Handle special moves: restore castling rights, en passant, promotion
        // 4. Restore game state (side to move, last move, etc.)
        let last_undo = self.undo_info.pop().expect("No move to unmake");
        let last_move = last_undo.last_move;

        self.can_castle = last_undo.can_castle;

        if last_move.flags == Move::FLAG_PROMOTION {
            if self.is_color(last_move.to, Color::White) {
                self.squares[last_move.to] = W_PAWN;
            } else {
                self.squares[last_move.to] = B_PAWN;
            }
        }
        if last_move.piece == PieceType::King {
            match self.side_to_move {
                Color::White => self.black_king_index = last_move.from,
                Color::Black => self.white_king_index = last_move.from,
            }
        }
        self.squares[last_move.from] = self.squares[last_move.to]; // Move piece back
        //
        self.squares[last_move.to] =
            if last_move.captured != PieceType::None && last_move.flags != Move::FLAG_EN_PASSANT {
                match last_move.captured {
                    PieceType::Pawn => {
                        if self.side_to_move == Color::White {
                            W_PAWN
                        } else {
                            B_PAWN
                        }
                    }
                    PieceType::Knight => {
                        if self.side_to_move == Color::White {
                            W_KNIGHT
                        } else {
                            B_KNIGHT
                        }
                    }
                    PieceType::Bishop => {
                        if self.side_to_move == Color::White {
                            W_BISHOP
                        } else {
                            B_BISHOP
                        }
                    }
                    PieceType::Rook => {
                        if self.side_to_move == Color::White {
                            W_ROOK
                        } else {
                            B_ROOK
                        }
                    }
                    PieceType::Queen => {
                        if self.side_to_move == Color::White {
                            W_QUEEN
                        } else {
                            B_QUEEN
                        }
                    }
                    PieceType::King => {
                        if self.side_to_move == Color::White {
                            W_KING
                        } else {
                            B_KING
                        }
                    }
                    _ => EMPTY,
                }
            } else {
                EMPTY
            };
        match last_move.flags {
            Move::FLAG_EN_PASSANT => {
                if self.side_to_move == Color::White {
                    self.squares[last_move.to + 10] = W_PAWN;
                } else {
                    self.squares[last_move.to - 10] = B_PAWN;
                }
            }
            Move::FLAG_CASTLE_KING | Move::FLAG_CASTLE_QUEEN => match self.side_to_move {
                Color::Black => {
                    if last_move.flags == Move::FLAG_CASTLE_KING {
                        self.squares[H1] = W_ROOK;
                        self.squares[F1] = EMPTY;
                    } else {
                        self.squares[A1] = W_ROOK;
                        self.squares[D1] = EMPTY;
                    }
                    self.white_king_index = last_move.from;
                }
                Color::White => {
                    if last_move.flags == Move::FLAG_CASTLE_KING {
                        self.squares[H8] = B_ROOK;
                        self.squares[F8] = EMPTY;
                    } else {
                        self.squares[A8] = B_ROOK;
                        self.squares[D8] = EMPTY;
                    }
                    self.black_king_index = last_move.from;
                }
            },
            _ => {}
        }

        self.side_to_move = self.side_to_move.opponent();
    }
}
