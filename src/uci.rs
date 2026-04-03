use crate::board::types::{Color, Move, PieceType};
use crate::board::Board;
use crate::fen;
use crate::movegen::{generate_moves, is_square_attacked, perft};
use serde::Serialize;
use std::io::{self, BufRead, Write};

#[derive(Serialize)]
struct BoardState {
    fen: String,
    squares: Vec<Option<PieceInfo>>,
    turn: String,
    castling_rights: String,
    en_passant: Option<String>,
    white_king: usize,
    black_king: usize,
    last_move: Option<MoveInfo>,
}

#[derive(Serialize)]
struct PieceInfo {
    piece: String,
    color: String,
}

#[derive(Serialize)]
struct MoveInfo {
    from: String,
    to: String,
    piece: String,
    captured: Option<String>,
    promotion: Option<String>,
}

#[derive(Serialize)]
struct LegalMovesResponse {
    moves: Vec<MoveDetail>,
    count: usize,
}

#[derive(Serialize)]
struct MoveDetail {
    from: String,
    to: String,
    san: String,
    piece: String,
    flags: u8,
}

#[derive(Serialize)]
struct PerftResult {
    depth: u32,
    nodes: u64,
    time_ms: u128,
}

#[derive(Serialize)]
struct BestMoveResult {
    best_move: String,
    eval: i32,
    depth: i32,
    nodes: u64,
    time_ms: u128,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
    message: String,
}

pub struct UciEngine {
    board: Board,
}

impl UciEngine {
    pub fn new() -> Self {
        let board = fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        UciEngine { board }
    }

    pub fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            let response = match parts[0] {
                "uci" => self.cmd_uci(),
                "isready" => self.cmd_isready(),
                "position" => self.cmd_position(&parts),
                "moves" => self.cmd_moves(),
                "go" => self.cmd_go(&parts),
                "perft" => self.cmd_perft(&parts),
                "d" => self.cmd_display(),
                "move" => self.cmd_make_move(&parts),
                "undo" => self.cmd_undo(),
                "state" => self.cmd_state(),
                "quit" => {
                    let _ = writeln!(stdout, "{{\"status\":\"quit\"}}");
                    let _ = stdout.flush();
                    break;
                }
                _ => serde_json::to_string(&ErrorResponse { error: format!("Unknown command: {}", parts[0]) }).unwrap(),
            };

            let _ = writeln!(stdout, "{}", response);
            let _ = stdout.flush();
        }
    }

    fn cmd_uci(&self) -> String {
        serde_json::to_string(&SuccessResponse { success: true, message: "rust_chess UCI ready".to_string() }).unwrap()
    }

    fn cmd_isready(&self) -> String {
        serde_json::to_string(&SuccessResponse { success: true, message: "readyok".to_string() }).unwrap()
    }

    fn cmd_position(&mut self, parts: &[&str]) -> String {
        if parts.len() < 2 {
            return serde_json::to_string(&ErrorResponse {
                error: "Usage: position [fen <fenstring> | startpos] [moves ...]".to_string(),
            })
            .unwrap();
        }

        if parts[1] == "startpos" {
            match fen::fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1") {
                Ok(board) => self.board = board,
                Err(e) => return serde_json::to_string(&ErrorResponse { error: e }).unwrap(),
            }
        } else if parts[1] == "fen" && parts.len() >= 3 {
            let fen = parts[2..].join(" ");
            match fen::fen_to_board(&fen) {
                Ok(board) => self.board = board,
                Err(e) => return serde_json::to_string(&ErrorResponse { error: e }).unwrap(),
            }
        } else {
            return serde_json::to_string(&ErrorResponse { error: "Invalid position command".to_string() }).unwrap();
        }

        serde_json::to_string(&SuccessResponse { success: true, message: "Position set".to_string() }).unwrap()
    }

    fn cmd_moves(&self) -> String {
        let moves = generate_moves(&self.board);
        let mut legal_moves = Vec::new();

        for mv in moves {
            // Verificar si el movimiento deja al rey en jaque
            let mut test_board = Board::new(
                self.board.squares,
                self.board.side_to_move,
                self.board.undo_info.clone(),
                self.board.can_castle,
                self.board.white_king_index,
                self.board.black_king_index,
            );
            test_board.make_move(mv);

            let king_sq = if self.board.side_to_move == Color::White {
                test_board.white_king_index
            } else {
                test_board.black_king_index
            };

            if !is_square_attacked(&test_board, king_sq, self.board.side_to_move.opponent()) {
                legal_moves.push(MoveDetail {
                    from: index_to_square(mv.from),
                    to: index_to_square(mv.to),
                    san: move_to_san(mv, &self.board),
                    piece: piece_type_to_string(mv.piece),
                    flags: mv.flags,
                });
            }
        }

        let count = legal_moves.len();
        serde_json::to_string(&LegalMovesResponse { moves: legal_moves, count }).unwrap()
    }

    fn cmd_go(&self, _parts: &[&str]) -> String {
        use std::time::Instant;

        let start = Instant::now();
        let moves = generate_moves(&self.board);
        let mut legal_moves = Vec::new();

        // Filtrar movimientos legales
        for mv in moves {
            let mut test_board = Board::new(
                self.board.squares,
                self.board.side_to_move,
                self.board.undo_info.clone(),
                self.board.can_castle,
                self.board.white_king_index,
                self.board.black_king_index,
            );
            test_board.make_move(mv);

            let king_sq = if self.board.side_to_move == Color::White {
                test_board.white_king_index
            } else {
                test_board.black_king_index
            };

            if !is_square_attacked(&test_board, king_sq, self.board.side_to_move.opponent()) {
                legal_moves.push(mv);
            }
        }

        if legal_moves.is_empty() {
            return serde_json::to_string(&ErrorResponse { error: "No legal moves".to_string() }).unwrap();
        }

        // Seleccionar movimiento aleatorio
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as usize;
        let index = seed % legal_moves.len();
        let best = legal_moves[index];
        let eval = 0; // TODO: implementar evaluación
        let nodes = legal_moves.len() as u64;
        let time_ms = start.elapsed().as_millis();

        serde_json::to_string(&BestMoveResult {
            best_move: format!("{}{}", index_to_square(best.from), index_to_square(best.to)),
            eval,
            depth: 1,
            nodes,
            time_ms,
        })
        .unwrap()
    }

    fn cmd_perft(&self, parts: &[&str]) -> String {
        use std::time::Instant;

        if parts.len() < 2 {
            return serde_json::to_string(&ErrorResponse { error: "Usage: perft <depth>".to_string() }).unwrap();
        }

        let depth: u32 = match parts[1].parse() {
            Ok(d) => d,
            Err(_) => return serde_json::to_string(&ErrorResponse { error: "Invalid depth".to_string() }).unwrap(),
        };

        let mut test_board = Board::new(
            self.board.squares,
            self.board.side_to_move,
            self.board.undo_info.clone(),
            self.board.can_castle,
            self.board.white_king_index,
            self.board.black_king_index,
        );

        let start = Instant::now();
        let nodes = perft(&mut test_board, depth);
        let time_ms = start.elapsed().as_millis();

        serde_json::to_string(&PerftResult { depth, nodes, time_ms }).unwrap()
    }

    fn cmd_display(&self) -> String {
        self.board.print();
        serde_json::to_string(&SuccessResponse { success: true, message: "Board printed to stdout".to_string() })
            .unwrap()
    }

    fn cmd_make_move(&mut self, parts: &[&str]) -> String {
        if parts.len() < 2 {
            return serde_json::to_string(&ErrorResponse {
                error: "Usage: move <from><to> (e.g., move e2e4)".to_string(),
            })
            .unwrap();
        }

        let move_str = parts[1];
        if move_str.len() != 4 {
            return serde_json::to_string(&ErrorResponse { error: "Invalid move format. Use e2e4 format".to_string() })
                .unwrap();
        }

        let from = square_to_index(&move_str[0..2]);
        let to = square_to_index(&move_str[2..4]);

        let moves = generate_moves(&self.board);
        let mut found_move = None;

        for mv in moves {
            if mv.from == from && mv.to == to {
                // Verificar si es legal
                let mut test_board = Board::new(
                    self.board.squares,
                    self.board.side_to_move,
                    self.board.undo_info.clone(),
                    self.board.can_castle,
                    self.board.white_king_index,
                    self.board.black_king_index,
                );
                test_board.make_move(mv);

                let king_sq = if self.board.side_to_move == Color::White {
                    test_board.white_king_index
                } else {
                    test_board.black_king_index
                };

                if !is_square_attacked(&test_board, king_sq, self.board.side_to_move.opponent()) {
                    found_move = Some(mv);
                    break;
                }
            }
        }

        match found_move {
            Some(mv) => {
                self.board.make_move(mv);
                serde_json::to_string(&SuccessResponse {
                    success: true,
                    message: format!("Move played: {}{}", index_to_square(mv.from), index_to_square(mv.to)),
                })
                .unwrap()
            }
            None => serde_json::to_string(&ErrorResponse { error: "Illegal move".to_string() }).unwrap(),
        }
    }

    fn cmd_undo(&mut self) -> String {
        if self.board.undo_info.is_empty() {
            return serde_json::to_string(&ErrorResponse { error: "No moves to undo".to_string() }).unwrap();
        }

        self.board.unmake_move();
        serde_json::to_string(&SuccessResponse { success: true, message: "Move undone".to_string() }).unwrap()
    }

    fn cmd_state(&self) -> String {
        let mut squares = Vec::with_capacity(64);

        for rank in (2..10).rev() {
            for file in 1..9 {
                let idx = rank * 10 + file;
                let piece = self.board.squares[idx];

                if piece == 0 || piece == 0xFF {
                    squares.push(None);
                } else {
                    let color = if (piece & 0x80) != 0 { "black" } else { "white" };
                    let piece_type = match piece & 0x07 {
                        1 => "pawn",
                        2 => "knight",
                        3 => "bishop",
                        4 => "rook",
                        5 => "queen",
                        6 => "king",
                        _ => "none",
                    };
                    squares.push(Some(PieceInfo { piece: piece_type.to_string(), color: color.to_string() }));
                }
            }
        }

        let last_move =
            self.board.undo_info.last().filter(|info| info.last_move.piece != PieceType::None).map(|info| MoveInfo {
                from: index_to_square(info.last_move.from),
                to: index_to_square(info.last_move.to),
                piece: piece_type_to_string(info.last_move.piece),
                captured: if info.last_move.captured != PieceType::None {
                    Some(piece_type_to_string(info.last_move.captured))
                } else {
                    None
                },
                promotion: if info.last_move.promotion != PieceType::None {
                    Some(piece_type_to_string(info.last_move.promotion))
                } else {
                    None
                },
            });

        let en_passant = self.board.undo_info.last().and_then(|u| u.en_passant_square).map(index_to_square);

        let castling = format!("{:04b}", self.board.can_castle);

        serde_json::to_string(&BoardState {
            fen: fen::board_to_fen(&self.board),
            squares,
            turn: if self.board.side_to_move == Color::White { "white".to_string() } else { "black".to_string() },
            castling_rights: castling,
            en_passant,
            white_king: self.board.white_king_index,
            black_king: self.board.black_king_index,
            last_move,
        })
        .unwrap()
    }
}

fn square_to_index(sq: &str) -> usize {
    if sq.len() != 2 {
        return 0;
    }
    let file = sq.chars().nth(0).unwrap() as usize - 'a' as usize + 1;
    let rank = sq.chars().nth(1).unwrap() as usize - '0' as usize + 1;
    rank * 10 + file
}

fn index_to_square(idx: usize) -> String {
    let file = (idx % 10) as u8 - 1 + b'a';
    let rank = (idx / 10) as u8 - 1;
    format!("{}{}", file as char, rank)
}

fn piece_type_to_string(pt: PieceType) -> String {
    match pt {
        PieceType::Pawn => "pawn".to_string(),
        PieceType::Knight => "knight".to_string(),
        PieceType::Bishop => "bishop".to_string(),
        PieceType::Rook => "rook".to_string(),
        PieceType::Queen => "queen".to_string(),
        PieceType::King => "king".to_string(),
        PieceType::None => "none".to_string(),
    }
}

fn move_to_san(mv: Move, _board: &Board) -> String {
    // Simplificado - sin notación completa SAN
    format!("{}{}", index_to_square(mv.from), index_to_square(mv.to))
}
