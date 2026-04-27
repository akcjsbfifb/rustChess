# rustChess

Motor de ajedrez escrito en Rust con interfaz web y herramientas de testing.

## Cómo empezar

### Jugar en el navegador
```bash
cd web
go run server.go
# Abrir http://localhost:8080
```

### Usar el motor desde consola
```bash
cargo run --release
# Escribir: go depth 4
```

### Comparar dos versiones
```bash
cd benchmark
python3 match.py --engine1 ../target/release/rust_chess \
                 --engine2 ../target/release/rust_chess \
                 --games 100
```

## Estructura del proyecto

```
rust_chess/
├── src/           # Motor en Rust (búsqueda, evaluación, generación de movimientos)
├── web/           # Servidor Go + UI web con WebSocket
├── benchmark/     # Herramientas Python para testing y medición Elo
└── docs/          # Documentación técnica
```

## Estrategias del motor

### Algoritmo de búsqueda
- **Negamax** con **Alpha-Beta Pruning** - Optimización estándar de minimax
- **Profundidad**: Configurable (default 4-6 plies)
- **Ordenamiento de movimientos**: Básico (planificado: MVV-LVA)
- **Búsqueda quiescence**: Evaluación estática nada más (planificado: búsqueda selectiva en hojas)

### Función de evaluación
- **Material**: Valores de piezas (Peón=100, Caballo=320, Alfil=330, Torre=500, Dama=900)
- **Tablas de piezas (PST)**: Bonus/malus por posición
  - Peones: Control del centro y avance
  - Caballos: Preferir casillas centrales
  - Reyes: Seguridad en apertura/medio juego
- **Movilidad**: (Planificado) Contar movimientos legales
- **Seguridad del rey**: (Planificado) Escudo de peones y ataques

### Generación de movimientos
- **Tablero mailbox (10x12)**: Detección rápida de fuera de tablero
- **Filtro de movimientos legales**: Pseudo-legales + validación de jaque
- **Actualizaciones incrementales**: Aplicar/deshacer sin regenerar todo

### Optimizaciones futuras
- **Bitboards**: (Planificado) Generación más rápida de ataques
- **Tabla de transposición**: (Planificado) Zobrist hashing + cache
- **Iterative deepening**: (Planificado) Manejo de tiempo
- **Null move pruning**: (Planificado) Cortes más rápidos

## Componentes

| Componente | Tecnología | Propósito |
|------------|------------|-----------|
| **Motor** | Rust | Lógica del juego, algoritmos de búsqueda |
| **Servidor web** | Go + Gorilla WebSocket | UI en tiempo real |
| **Frontend** | JS vanilla + TailwindCSS | Tablero, entrada de movimientos |
| **Benchmark** | Python 3 | Testing estadístico, cálculo Elo |
| **Protocolo** | JSON sobre WebSocket | Comunicación motor-servidor |

## Estado del motor

- ✅ Movimientos de todas las piezas
- ✅ Movimientos especiales: Enroque, al paso, promoción
- ✅ Validación: Movimientos legales, detección de jaque
- ✅ Búsqueda: Negamax con poda alpha-beta
- ✅ Evaluación: Material + tablas de posición
- ✅ Protocolo UCI: Compatible con interfaces estándar
- 🔄 Ordenamiento de movimientos: MVV-LVA (en progreso)
- 🔄 Tablas de transposición (planificado)
- 🔄 Bitboards (planificado)

## Representación del tablero

El motor usa un **tablero mailbox de 10x12** (120 casillas) para detectar rápido cuando una pieza sale del tablero:

```
FFFFFFFFFFFFFFFFFFFF  ← Borde (0xFF)
FFFFFFFFFFFFFFFFFFFF  ← Borde
FF0402030506030204FF  ← Primera fila
FF0101010101010101FF  ← Peones
FF0000000000000000FF  ← Vacío
FF0000000000000000FF  ← Vacío
FF0000000000000000FF  ← Vacío
FF0000000000000000FF  ← Vacío
FF8181818181818181FF  ← Peones (negros)
FF8482838586838284FF  ← Primera fila (negros)
FFFFFFFFFFFFFFFFFFFF  ← Borde
FFFFFFFFFFFFFFFFFFFF  ← Borde
```

### Codificación de piezas

| Bits | Significado |
|------|-------------|
| `0-2` | Tipo (0=vacío, 1=peón, 2=caballo, 3=alfil, 4=torre, 5=dama, 6=rey) |
| `3` | Flag de "ya se movió" |
| `4` | Flag de enroque (reyes) |
| `7` | Color (0=blanco, 1=negro) |

## Navegación

| Carpeta | Contenido |
|---------|-----------|
| [src/](src/) | Código fuente Rust |
| [web/](web/) | Servidor web y archivos estáticos |
| [benchmark/](benchmark/) | Herramientas Python de testing |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Documentación técnica detallada |

## Desarrollo

### Requisitos
- Rust 1.70+ (`cargo`)
- Go 1.21+ (para UI web)
- Python 3.10+ (para benchmarks)

### Compilar
```bash
# Compilar motor
cargo build --release

# Compilar servidor web
cd web && go build

# Correr tests
cargo test
python3 -m pytest benchmark/
```

### Modo desarrollo
```bash
# Terminal 1: Motor
cargo run

# Terminal 2: Servidor web
cd web && go run server.go

# Navegador: http://localhost:8080
```

## Licencia

MIT License

## Créditos

- Técnicas de bitboard inspiradas en [Chess Programming Wiki](https://www.chessprogramming.org/)
- WebSocket usando [Gorilla](https://github.com/gorilla/websocket)
- UI con [TailwindCSS](https://tailwindcss.com)
