# rustChess ♟️

A chess engine written in Rust with web UI and benchmarking tools.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org)
[![Go](https://img.shields.io/badge/Go-1.21%2B-blue)](https://golang.org)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

## 🚀 Quick Start

### Play Online
```bash
cd web
go run server.go
# Open http://localhost:8080
```

### Run Engine CLI
```bash
cargo run --release
# Then type: go depth 4
```

### Benchmark Two Versions
```bash
cd benchmark
python3 match.py --engine1 ../target/release/rust_chess \
                 --engine2 ../target/release/rust_chess \
                 --games 100
```

## 🏗️ Architecture

```
rust_chess/
├── src/           # Rust engine (negamax, alpha-beta, move generation)
├── web/           # Go web server + WebSocket real-time UI
├── benchmark/     # Python tools for testing and Elo measurement
└── docs/          # Architecture diagrams and guides
```

## 🧠 Engine Strategies

### Search Algorithm
- **Negamax** with **Alpha-Beta Pruning** - Standard minimax optimization
- **Depth**: Configurable (default 4-6 plies)
- **Move Ordering**: No advanced ordering yet (planned: MVV-LVA)
- **Quiescence Search**: Static evaluation only (planned: selective search at leaf nodes)

### Evaluation Function
- **Material**: Piece values (Pawn=100, Knight=320, Bishop=330, Rook=500, Queen=900)
- **Piece-Square Tables (PST)**: Bonus/malus for piece positioning
  - Pawns: Encourage center control and advancement
  - Knights: Prefer central outposts
  - Kings: Safety in early/mid game
- **Mobility**: (Planned) Count legal moves
- **King Safety**: (Planned) Pawn shield and piece attacks

### Move Generation
- **Mailbox Board (10x12)**: Fast off-board detection with border squares
- **Legal Move Filter**: Pseudo-legal generation + check validation
- **Incremental Updates**: Apply/undo moves without full regeneration

### Optimizations
- **Bitboards**: (Planned) For faster attack generation
- **Transposition Table**: (Planned) Zobrist hashing + cache
- **Iterative Deepening**: (Planned) Time management
- **Null Move Pruning**: (Planned) For faster cutoffs

### Tech Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Engine** | Rust | Core chess logic, search algorithms |
| **Web Server** | Go + Gorilla WebSocket | Real-time multiplayer UI |
| **Frontend** | Vanilla JS + TailwindCSS | Chess board, move input |
| **Benchmark** | Python 3 | Statistical testing, Elo calculation |
| **Protocol** | JSON over WebSocket | Engine communication |

## 🎯 Features

### Chess Engine
- ✅ **All piece movement** (pawns, knights, bishops, rooks, queens, kings)
- ✅ **Special moves**: Castling, en passant, promotion
- ✅ **Move validation**: Legal move generation, check detection
- ✅ **Search**: Negamax with alpha-beta pruning
- ✅ **Evaluation**: Material + position tables (PST)
- ✅ **UCI Protocol**: Compatible with standard chess interfaces

### Web Interface
- 🌐 **Real-time multiplayer**: WebSocket sync
- 📱 **Responsive**: Works on desktop and mobile
- ⚡ **Engine vs Human**: Play against the AI
- 🔍 **Debug mode**: See engine thinking
- 📊 **Benchmark UI**: Compare any two git commits visually

### Benchmarking
- 🔄 **Automated matches**: Test any two commits
- 📈 **Elo calculation**: Statistical significance testing
- 🧪 **Openings**: Test with standard EPD positions
- 📉 **SPRTest**: Early termination when significance reached

## 🎮 Board Representation

The engine uses a **10x12 mailbox board** (120 squares) for fast off-board detection:

```
FFFFFFFFFFFFFFFFFFFF  ← Border (0xFF)
FFFFFFFFFFFFFFFFFFFF  ← Border
FF0402030506030204FF  ← Back rank
FF0101010101010101FF  ← Pawns
FF0000000000000000FF  ← Empty
FF0000000000000000FF  ← Empty
FF0000000000000000FF  ← Empty
FF0000000000000000FF  ← Empty
FF8181818181818181FF  ← Pawns (black)
FF8482838586838284FF  ← Back rank (black)
FFFFFFFFFFFFFFFFFFFF  ← Border
FFFFFFFFFFFFFFFFFFFF  ← Border
```

### Piece Encoding

| Bits | Meaning |
|------|---------|
| `0-2` | Piece type (0=empty, 1=pawn, 2=knight, 3=bishop, 4=rook, 5=queen, 6=king) |
| `3` | Has moved flag |
| `4` | Castle rights (kings) |
| `7` | Color (0=white, 1=black) |

## 📁 Project Structure

| Path | Description |
|------|-------------|
| [src/](src/) | Rust engine source |
| [web/](web/) | Go web server and static files |
| [benchmark/](benchmark/) | Python benchmarking tools |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Technical deep dive |
| [docs/BENCHMARK_GUIDE.md](docs/BENCHMARK_GUIDE.md) | How to benchmark |

## 🛠️ Development

### Prerequisites
- Rust 1.70+ (`cargo`)
- Go 1.21+ (for web UI)
- Python 3.10+ (for benchmarks)

### Build
```bash
# Build engine
cargo build --release

# Build web server
cd web && go build

# Run tests
cargo test
python3 -m pytest benchmark/
```

### Development Mode
```bash
# Terminal 1: Run engine
cargo run

# Terminal 2: Run web server
cd web && go run server.go

# Browser: http://localhost:8080
```

## 📊 Benchmarking Guide

See [docs/BENCHMARK_GUIDE.md](docs/BENCHMARK_GUIDE.md) for detailed instructions on comparing engine versions.

Quick example:
```bash
# Compare current HEAD vs 5 commits ago
cd benchmark
python3 gitbench.py --vs-commit HEAD~5 --games 100
```

## 📝 License

MIT License - see LICENSE file for details.

## 🙏 Credits

- Bitboard techniques inspired by [Chess Programming Wiki](https://www.chessprogramming.org/)
- WebSocket implementation using [Gorilla](https://github.com/gorilla/websocket)
- UI styled with [TailwindCSS](https://tailwindcss.com)

---

**Status**: Active development - PRs welcome! 🎉
