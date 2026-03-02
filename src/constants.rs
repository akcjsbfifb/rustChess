pub const BOARD_SIZE: usize = 120;
pub const EMPTY: u8 = 0;
pub const OFF_BOARD: u8 = 0xFF;

// Bits de la representación de pieza
pub const COLOR_BIT: u8 = 0b1000_0000; // Bit 7: 1=Negro, 0=Blanco
pub const MOVED_BIT: u8 = 0b0000_1000; // Bit 3: Se ha movido
pub const CASTLE_BIT: u8 = 0b0001_0000; // Bit 4: Flag de enroque
pub const TYPE_MASK: u8 = 0b0000_0111; // Bits 2-0: Tipo de pieza

// Piezas Blancas
pub const W_PAWN: u8 = 1;
pub const W_KNIGHT: u8 = 2;
pub const W_BISHOP: u8 = 3;
pub const W_ROOK: u8 = 4;
pub const W_QUEEN: u8 = 5;
pub const W_KING: u8 = 6;

// Piezas Negras
pub const B_PAWN: u8 = COLOR_BIT | 1;
pub const B_KNIGHT: u8 = COLOR_BIT | 2;
pub const B_BISHOP: u8 = COLOR_BIT | 3;
pub const B_ROOK: u8 = COLOR_BIT | 4;
pub const B_QUEEN: u8 = COLOR_BIT | 5;
pub const B_KING: u8 = COLOR_BIT | 6;

// Casillas (A1-H8) para representación 10x12
pub const A1: usize = 21;
pub const B1: usize = 22;
pub const C1: usize = 23;
pub const D1: usize = 24;
pub const E1: usize = 25;
pub const F1: usize = 26;
pub const G1: usize = 27;
pub const H1: usize = 28;

pub const A2: usize = 31;
pub const B2: usize = 32;
pub const C2: usize = 33;
pub const D2: usize = 34;
pub const E2: usize = 35;
pub const F2: usize = 36;
pub const G2: usize = 37;
pub const H2: usize = 38;

pub const A3: usize = 41;
pub const B3: usize = 42;
pub const C3: usize = 43;
pub const D3: usize = 44;
pub const E3: usize = 45;
pub const F3: usize = 46;
pub const G3: usize = 47;
pub const H3: usize = 48;

pub const A4: usize = 51;
pub const B4: usize = 52;
pub const C4: usize = 53;
pub const D4: usize = 54;
pub const E4: usize = 55;
pub const F4: usize = 56;
pub const G4: usize = 57;
pub const H4: usize = 58;

pub const A5: usize = 61;
pub const B5: usize = 62;
pub const C5: usize = 63;
pub const D5: usize = 64;
pub const E5: usize = 65;
pub const F5: usize = 66;
pub const G5: usize = 67;
pub const H5: usize = 68;

pub const A6: usize = 71;
pub const B6: usize = 72;
pub const C6: usize = 73;
pub const D6: usize = 74;
pub const E6: usize = 75;
pub const F6: usize = 76;
pub const G6: usize = 77;
pub const H6: usize = 78;

pub const A7: usize = 81;
pub const B7: usize = 82;
pub const C7: usize = 83;
pub const D7: usize = 84;
pub const E7: usize = 85;
pub const F7: usize = 86;
pub const G7: usize = 87;
pub const H7: usize = 88;

pub const A8: usize = 91;
pub const B8: usize = 92;
pub const C8: usize = 93;
pub const D8: usize = 94;
pub const E8: usize = 95;
pub const F8: usize = 96;
pub const G8: usize = 97;
pub const H8: usize = 98;
