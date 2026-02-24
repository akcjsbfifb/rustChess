# rustChess

Chess engine written in Rust.

## Board Representation

The board is represented using a 120-square array (10x12). The value `0xFF` forms the border of the board and is used to detect when a piece moves off the board.

The board array looks like:

    FFFFFFFFFFFFFFFFFFFF
    FFFFFFFFFFFFFFFFFFFF
    FF0402030506030204FF
    FF0101010101010101FF
    FF0000000000000000FF
    FF0000000000000000FF
    FF0000000000000060FF
    FF0000000000000000FF
    FF8181818181818181FF
    FF8482838586838284FF
    FFFFFFFFFFFFFFFFFFFF
    FFFFFFFFFFFFFFFFFFFF

### Piece encoding (lower 3 bits)

| Value | Piece    |
|-------|----------|
| 0     | Empty    |
| 1     | Pawn     |
| 2     | Knight   |
| 3     | Bishop   |
| 4     | Rook     |
| 5     | Queen    |
| 6     | King     |

### Bit 7 - Color

- 0 = White
- 1 = Black

### Other bits

- Bit 3: Piece has moved flag
- Bit 4: Castle flag (King only)

## Status

- Pawn moves: implemented (single advance, double advance, captures, promotion)
- En passant: not implemented yet
- Castling: not implemented yet
- Other pieces: not implemented yet
