#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    None = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub piece: PieceType,
    pub captured: PieceType,
    pub promotion: PieceType,
    pub flags: u8,
}

impl Move {
    pub const FLAG_NONE: u8 = 0;
    pub const FLAG_CASTLE: u8 = 1;
    pub const FLAG_EN_PASSANT: u8 = 2;
    pub const FLAG_DOUBLE_PAWN: u8 = 3;

    pub fn new(
        from: usize,
        to: usize,
        piece: PieceType,
        captured: PieceType,
        promotion: PieceType,
        flags: u8,
    ) -> Self {
        Self {
            from,
            to,
            piece,
            captured,
            promotion,
            flags,
        }
    }
}
