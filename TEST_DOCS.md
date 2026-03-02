# Documentación para Tests - Chess Engine

## Reglas Importantes

- **NUNCA modificar código fuera de la carpeta `tests/`**
- **Si un test falla, el código debe arreglarse en `src/`, no el test**

---

## Tablero (Board)

### Representación

El tablero es un array unidimensional de 120 casillas (`BOARD_SIZE = 120`).

### Sistema de Índices

```
   a  b  c  d  e  f  g  h
8  91 92 93 94 95 96 97 98  <- Torre negra en 91 y 98
7  81 82 83 84 85 86 87 88
6  71 72 73 74 75 76 77 78
5  61 62 63 64 65 66 67 68
4  51 52 53 54 55 56 57 58
3  41 42 43 44 45 46 47 48
2  31 32 33 34 35 36 37 38
1  21 22 23 24 25 26 27 28  <- Torre blanca en 21 y 28
   11 12 13 14 15 16 17 18
```

- Las casillas 0-10, 11, 20, 30, 40, 50, 60, 70, 80, 90, 100-110 son `OFF_BOARD` (0xFF)
- El rango válido es 21-98 (excluyendo posiciones de borde)

### Valores de Piezas

| Pieza | Blanca | Negra |
|-------|-------|-------|
| Rey   | 0x06  | 0x86  |
| Reina | 0x05  | 0x85  |
| Torre | 0x04  | 0x84  |
| Alfil | 0x03  | 0x83  |
| Caballo| 0x02 | 0x82  |
| Peón  | 0x01  | 0x81  |

**Fórmula**: `pieza_negra = 0x80 | pieza_blanca`

Para obtener el tipo de pieza: `p & 0x0F`
Para obtener el color: `p & 0x80` (si es 0, es blanca)

### Constantes

```rust
pub const EMPTY: u8 = 0;
pub const OFF_BOARD: u8 = 0xFF;
pub const TYPE_MASK: u8 = 0x0F;
```

---

## Castling (Enroque)

### Flags

```rust
pub const FLAG_CASTLE_KING: u8 = 1;   // Enroque corto
pub const FLAG_CASTLE_QUEEN: u8 = 5; // Enroque largo
```

### Bitmask de Derechos de Enroque

```
bit 0 (1): White OO  (enroque corto blanco)
bit 1 (2): White OOO (enroque largo blanco)
bit 2 (4): Black OO  (enroque corto negro)
bit 3 (8): Black OOO (enroque largo negro)
```

- Inicial: `0b1111` (todos permitidos)
- Para quitar un derecho: usar AND con la máscara negada

**Ejemplos**:
- Quitar White OO: `can_castle &= 0b1110` (quitar bit 0)
- Quitar Black OO: `can_castle &= 0b1011` (quitar bit 2)

### Posiciones de Reyes y Torres

| Movimiento | Rey (from→to) | Torre (from→to) |
|------------|---------------|-----------------|
| White OO   | 25→27         | 28→26           |
| White OOO  | 25→23         | 21→24           |
| Black OO   | 95→97         | 98→96           |
| Black OOO  | 95→93         | 91→94           |

---

## Move (Movimiento)

### Estructura

```rust
Move::new(
    from,           // Casilla origen
    to,             // Casilla destino
    piece,          // Pieza que se mueve (PieceType)
    captured,       // Pieza capturada (PieceType::None si no hay)
    promotion,      // Pieza de promoción (PieceType::None si no hay)
    flags           // Flags especiales
)
```

### Flags de Movimiento

```rust
pub const FLAG_NONE: u8 = 0;
pub const FLAG_EN_PASSANT: u8 = 2;
pub const FLAG_CASTLE_KING: u8 = 1;
pub const FLAG_CASTLE_QUEEN: u8 = 5;
pub const FLAG_PROMOTION: u8 = 4;
```

---

## side_to_move (Bando a mover)

- Es de tipo `Color` (`White` o `Black`)
- **IMPORTANTE**: Al crear un `Board::new()`, `side_to_move` inicia como `Color::White`
- Si necesitas testar movimientos negros, **DEBES** cambiarlo:

```rust
let mut board = Board::new();
board.side_to_move = Color::Black;  // IMPORTANTE
```

---

## make_move y unmake_move

- `make_move(m)` ejecuta un movimiento y cambia `side_to_move`
- `unmake_move()` revierte el último movimiento y restaura `side_to_move`

---

## Color

```rust
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opponent(&self) -> Color { ... }
}
```

---

## PieceType

```rust
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
```

---

## Ejemplo de Test de Enroque

```rust
#[test]
fn test_black_kingside_castle_consistency() {
    let mut board = Board::new();
    board.side_to_move = Color::Black;  // ← IMPORTANTE
    board.squares[96] = EMPTY;         // Limpiar f8
    board.squares[97] = EMPTY;         // Limpiar g8
    let initial_squares = board.squares.clone();

    let m = Move::new(95, 97, PieceType::King, PieceType::None, PieceType::None, Move::FLAG_CASTLE_KING);

    board.make_move(m);
    assert_eq!(board.squares[97], 0x86, "El rey debería estar en g8");
    assert_eq!(board.squares[96], 0x84, "La torre debería estar en f8");

    board.unmake_move();
    assert_eq!(board.squares, initial_squares, "El tablero no se restauró correctamente");
}
```

---

## Errores Comunes

1. **Olvidar `board.side_to_move = Color::Black`**: El tablero siempre inicia con blancas
2. **Usar valores de piezas blancas para piezas negras**: Usar `0x86` en vez de `0x06`
3. **Índices incorrectos**: Verificar la tabla de índices выше
4. **No limpiar casillas de paso**: Antes de enrocar, limpiar f1/g1 o f8/g8
