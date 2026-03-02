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
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: INITIAL_POSITION,
            side_to_move: Color::White,
            undo_info: Vec::new(),
            can_castle: 0b1111, // Ambos lados pueden enrocarse
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
            Move::FLAG_CASTLE_KING | Move::FLAG_CASTLE_QUEEN => {
                match self.side_to_move {
                    Color::White => {
                        if _mv.flags == Move::FLAG_CASTLE_KING {
                            self.squares[28] = EMPTY;
                            self.squares[26] = 0x04; // Coloca la torre en f1
                        } else {
                            self.squares[21] = EMPTY;
                            self.squares[24] = 0x04; // Coloca la torre en d1
                        }
                    }
                    Color::Black => {
                        if _mv.flags == Move::FLAG_CASTLE_KING {
                            self.squares[98] = EMPTY;
                            self.squares[96] = 0x84; // Coloca la torre en f8
                        } else {
                            self.squares[91] = EMPTY;
                            self.squares[94] = 0x84; // Coloca la torre en d8
                        }
                    }
                }
            }
            Move::FLAG_PROMOTION => {
                // Promote pawn to chosen piece
                self.squares[_mv.to] = match _mv.promotion {
                    PieceType::Queen => {
                        if self.side_to_move == Color::White {
                            0x05
                        } else {
                            0x85
                        }
                    }
                    PieceType::Rook => {
                        if self.side_to_move == Color::White {
                            0x04
                        } else {
                            0x84
                        }
                    }
                    PieceType::Bishop => {
                        if self.side_to_move == Color::White {
                            0x03
                        } else {
                            0x83
                        }
                    }
                    PieceType::Knight => {
                        if self.side_to_move == Color::White {
                            0x02
                        } else {
                            0x82
                        }
                    }
                    _ => self.squares[_mv.from], // No promotion, keep original piece
                };
            }
            _ => {}
        }
        if _mv.piece == PieceType::King {
            // Update castling rights if king moves
            match self.side_to_move {
                Color::White => self.can_castle &= 0b1100, // Quitar derechos de enroque para blancas
                Color::Black => self.can_castle &= 0b0011, // Quitar derechos de enroque para negras
            }
        }
        if _mv.piece == PieceType::Rook {
            // Update castling rights if rook moves
            match self.side_to_move {
                Color::White => {
                    if _mv.from == 21 {
                        self.can_castle &= 0b1110; // Quitar derecho de enroque largo para blancas
                    } else if _mv.from == 28 {
                        self.can_castle &= 0b1101; // Quitar derecho de enroque corto para blancas
                    }
                }
                Color::Black => {
                    if _mv.from == 91 {
                        self.can_castle &= 0b1011; // Quitar derecho de enroque largo para negras
                    } else if _mv.from == 98 {
                        self.can_castle &= 0b0111; // Quitar derecho de enroque corto para negras
                    }
                }
            }
        }
        if _mv.captured == PieceType::Rook {
            // Update castling rights if rook is captured
            match self.side_to_move.opponent() {
                Color::White => {
                    if _mv.to == 21 {
                        self.can_castle &= 0b1101; // Quitar derecho de enroque largo para blancas
                    } else if _mv.to == 28 {
                        self.can_castle &= 0b1110; // Quitar derecho de enroque corto para blancas
                    }
                }
                Color::Black => {
                    if _mv.to == 91 {
                        self.can_castle &= 0b1011; // Quitar derecho de enroque largo para negras
                    } else if _mv.to == 98 {
                        self.can_castle &= 0b1011; // Quitar derecho de enroque corto para negras
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
                self.squares[last_move.to] = 0x01;
            } else {
                self.squares[last_move.to] = 0x81;
            }
        }
        self.squares[last_move.from] = self.squares[last_move.to]; // Move piece back
                                                                   //
        self.squares[last_move.to] =
            if last_move.captured != PieceType::None && last_move.flags != Move::FLAG_EN_PASSANT {
                // Restore captured piece
                match last_move.captured {
                    PieceType::Pawn => {
                        if self.side_to_move == Color::White {
                            0x01
                        } else {
                            0x81
                        }
                    }
                    PieceType::Knight => {
                        if self.side_to_move == Color::White {
                            0x02
                        } else {
                            0x82
                        }
                    }
                    PieceType::Bishop => {
                        if self.side_to_move == Color::White {
                            0x03
                        } else {
                            0x83
                        }
                    }
                    PieceType::Rook => {
                        if self.side_to_move == Color::White {
                            0x04
                        } else {
                            0x84
                        }
                    }
                    PieceType::Queen => {
                        if self.side_to_move == Color::White {
                            0x05
                        } else {
                            0x85
                        }
                    }
                    PieceType::King => {
                        if self.side_to_move == Color::White {
                            0x06
                        } else {
                            0x86
                        }
                    }
                    _ => EMPTY,
                }
            } else {
                EMPTY
            };
        match last_move.flags {
            Move::FLAG_EN_PASSANT => {
                // Restore captured pawn
                if self.side_to_move == Color::White {
                    self.squares[last_move.to + 10] = 0x01;
                } else {
                    self.squares[last_move.to - 10] = 0x81;
                }
            }
            Move::FLAG_CASTLE_KING | Move::FLAG_CASTLE_QUEEN => {
                match self.side_to_move {
                    Color::Black => {
                        if last_move.flags == Move::FLAG_CASTLE_KING {
                            self.squares[28] = 0x04; // Restaura torre en h1
                            self.squares[26] = EMPTY; // Limpia f1
                        } else {
                            self.squares[21] = 0x04; // Restaura torre en a1
                            self.squares[24] = EMPTY; // Limpia d1
                        }
                    }
                    Color::White => {
                        if last_move.flags == Move::FLAG_CASTLE_KING {
                            self.squares[98] = 0x84; // Restaura torre en h8
                            self.squares[96] = EMPTY; // Limpia f8
                        } else {
                            self.squares[91] = 0x84; // Restaura torre en a8
                            self.squares[94] = EMPTY; // Limpia d8
                        }
                    }
                }
            }
            _ => {}
        }

        self.side_to_move = self.side_to_move.opponent();
    }
}
