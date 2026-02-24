use super::Board;
use super::types::PieceType;
use crate::constants::*;

const RESET: &str = "\x1b[0m";

const BG_LIGHT: &str = "\x1b[48;2;238;238;210m";
const BG_DARK: &str = "\x1b[48;2;118;150;86m";

const FG_WHITE: &str = "\x1b[38;2;50;50;50m";
const FG_BLACK: &str = "\x1b[38;2;0;0;0m";

fn piece_to_unicode(piece: u8) -> &'static str {
    let piece_type = piece & TYPE_MASK;
    let is_black = (piece & COLOR_BIT) != 0;

    match piece_type {
        1 => {
            if is_black {
                "♟"
            } else {
                "♙"
            }
        }
        2 => {
            if is_black {
                "♞"
            } else {
                "♘"
            }
        }
        3 => {
            if is_black {
                "♝"
            } else {
                "♗"
            }
        }
        4 => {
            if is_black {
                "♜"
            } else {
                "♖"
            }
        }
        5 => {
            if is_black {
                "♛"
            } else {
                "♕"
            }
        }
        6 => {
            if is_black {
                "♚"
            } else {
                "♔"
            }
        }
        _ => " ",
    }
}

impl Board {
    pub fn print(&self) {
        println!("\n   +----------------+");

        for rank in (2..10).rev() {
            print!(" {} |", rank - 1);

            for file in 1..9 {
                let index = rank * 10 + file;
                let piece = self.squares[index];

                let is_light = (rank + file) % 2 == 1;
                let bg = if is_light { BG_LIGHT } else { BG_DARK };
                let fg = if (piece & COLOR_BIT) != 0 {
                    FG_BLACK
                } else {
                    FG_WHITE
                };

                let symbol = piece_to_unicode(piece);
                print!("{}{}{} {}", bg, fg, symbol, RESET);
            }
            println!("|");
        }

        println!("   +----------------+");
        println!("     a b c d e f g h ");
    }
}
