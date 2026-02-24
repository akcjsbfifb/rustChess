pub const BOARD_SIZE: usize = 120;
pub const EMPTY: u8 = 0;
pub const OFF_BOARD: u8 = 0xFF;

// Bits de la representación de pieza
pub const COLOR_BIT: u8 = 0b1000_0000; // Bit 7: 1=Negro, 0=Blanco
pub const MOVED_BIT: u8 = 0b0000_1000; // Bit 3: Se ha movido
pub const CASTLE_BIT: u8 = 0b0001_0000; // Bit 4: Flag de enroque
pub const TYPE_MASK: u8 = 0b0000_0111; // Bits 2-0: Tipo de pieza
