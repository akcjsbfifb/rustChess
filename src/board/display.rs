use super::Board;
use crate::constants::*;

const RESET: &str = "\x1b[0m";

// Board colors (Lichess-like theme)
const BG_LIGHT: &str = "\x1b[48;5;187m"; // Light beige (less bright)
const BG_DARK: &str = "\x1b[48;5;65m";

// Highlight colors for last move
const BG_LIGHT_HL: &str = "\x1b[48;5;228m";
const BG_DARK_HL: &str = "\x1b[48;5;150m";

// Piece colors
const FG_WHITE: &str = "\x1b[1;38;5;231m"; // Bold Bright White
const FG_BLACK: &str = "\x1b[1;38;5;16m"; // Bold Black

fn piece_to_unicode(piece: u8) -> &'static str {
    let piece_type = piece & TYPE_MASK;
    // We use solid pieces for both colors for a cleaner look on colored backgrounds.
    match piece_type {
        1 => "♟", // Pawn
        2 => "♞", // Knight
        3 => "♝", // Bishop
        4 => "♜", // Rook
        5 => "♛", // Queen
        6 => "♚", // King
        _ => " ",
    }
}

impl Board {
    pub fn print(&self) {
        println!();

        let last_from = self.undo_info.last().map(|info| info.last_move.from);
        let last_to = self.undo_info.last().map(|info| info.last_move.to);

        println!("    a  b  c  d  e  f  g  h");

        for rank in (2..10).rev() {
            let rank_num = rank - 1;
            print!(" {} ", rank_num);

            for file in 1..9 {
                let index = rank * 10 + file;
                let piece = self.squares[index];

                let is_light = (rank + file) % 2 == 0;
                let is_highlighted = Some(index) == last_from || Some(index) == last_to;

                let bg = match (is_light, is_highlighted) {
                    (true, true) => BG_LIGHT_HL,
                    (false, true) => BG_DARK_HL,
                    (true, false) => BG_LIGHT,
                    (false, false) => BG_DARK,
                };

                let fg = if (piece & COLOR_BIT) != 0 { FG_BLACK } else { FG_WHITE };

                let symbol = piece_to_unicode(piece);
                print!("{}{} {} ", bg, fg, symbol);
            }
            println!("{} {} ", RESET, rank_num);
        }

        println!("    a  b  c  d  e  f  g  h");

        let turn = match self.side_to_move {
            super::types::Color::White => "White",
            super::types::Color::Black => "Black",
        };
        println!("\nTurn: {}", turn);
    }
}
