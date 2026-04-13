# Arquitectura Web Server + Propuesta de Benchmark UI

## Arquitectura Actual

### 1. Backend (Go - server.go)

**Estructura:**
```
Cliente WebSocket → Go Server → EngineProcess (Rust) → Respuesta JSON
```

**Message Protocol (WebSocket):**

Request (Frontend → Go):
```json
{
  "type": "engine_go",
  "payload": {"depth": 4}
}
```

Response (Go → Frontend):
```json
{
  "type": "best_move",
  "payload": {
    "best_move": "e2e4",
    "eval": 35,
    "depth": 4,
    "nodes": 1234,
    "time_ms": 100
  }
}
```

**Tipos de mensajes soportados:**
- `init` - Inicializar posición
- `get_moves` - Obtener movimientos legales
- `make_move` - Jugar movimiento
- `undo` - Deshacer
- `engine_go` - Motor juega (con depth)
- `perft` - Perft test
- `get_state` - Estado del tablero

**Problema:** El servidor solo puede comunicarse con UN motor (singleton pattern).

---

### 2. Frontend (HTML + Tailwind + JS)

**Páginas:**
- `index.html` - Tablero de ajedrez interactivo
- `debug.html` - Debug info, perft test, log de comunicación

**WebSocket Client (js/websocket.js):**
```javascript
class ChessWebSocket {
  send(message) → Envía JSON al Go server
  on(type, handler) → Registra handlers por tipo de mensaje
  handleMessage(data) → Recibe respuestas y dispara handlers
}
```

**Comunicación:**
- Conexión WebSocket a `ws://localhost:8080/ws`
- Protocolo bidireccional JSON
- Auto-reconnect en desconexión

---

## Propuesta: Benchmark Comparison UI

### Opción A: Nueva página benchmark.html (Recomendada)

**Ventajas:**
- No toca código existente (index.html/debug.html)
- Espacio dedicado para funcionalidad compleja
- No rompe nada que funciona

**Diseño propuesto:**

```html
benchmark.html
├── Header (navegación: Jugar | Debug | Benchmark)
├── Panel Principal
│   ├── Sección: Selección de Commits
│   │   ├── Dropdown: Commit A (default: HEAD)
│   │   ├── Dropdown: Commit B (default: HEAD~1)
│   │   └── Botón: "Cargar Commits Disponibles"
│   │
│   ├── Sección: Configuración
│   │   ├── Input: Número de partidas (default: 50)
│   │   ├── Input: Tiempo por movimiento (ms)
│   │   └── Checkbox: Guardar builds (--keep-builds)
│   │
│   ├── Sección: Progreso
│   │   ├── Progress bar: Partidas completadas
│   │   ├── Stats: Victorias A / Empates / Victorias B
│   │   └── Log en tiempo real: "Partida 23/50 - A gana"
│   │
│   └── Sección: Resultados
│       ├── Elo Difference: +XXX ± YY
│       ├── Intervalo 95%: [AAA, BBB]
│       ├── LOS: ZZZ%
│       └── Conclusión: Significativamente mejor/peor/inconcluso
│
└── WebSocket Status
```

---

### Implementación Técnica

#### 1. Backend - Nuevos Message Types (server.go)

```go
// Nuevos tipos de mensajes para benchmark

case "get_commits":
    // Ejecuta: git log --oneline -20
    // Retorna lista de commits para el dropdown
    commits := getGitCommits(20)
    c.conn.WriteJSON(Response{
        Type: "commits_list",
        Payload: commits,
    })

case "start_benchmark":
    var payload struct {
        CommitA  string `json:"commit_a"`  // Hash del commit A
        CommitB  string `json:"commit_b"`  // Hash del commit B
        Games    int    `json:"games"`       // Número de partidas
        TimeMs   int    `json:"time_ms"`     // Tiempo por movimiento
        KeepBuilds bool `json:"keep_builds"`
    }
    
    // Ejecutar benchmark en goroutine (no bloquear)
    go runBenchmark(payload.CommitA, payload.CommitB, payload.Games, c)
    
    c.conn.WriteJSON(Response{
        Type: "benchmark_started",
        Payload: map[string]interface{}{
            "games": payload.Games,
            "commit_a": payload.CommitA,
            "commit_b": payload.CommitB,
        },
    })
```

**Problema CRÍTICO:** El servidor actual usa un singleton EngineProcess compartido.
Para benchmark necesitamos DOS motores simultáneamente.

**Soluciones posibles:**

**Solución 1: Benchmark Engines separados (Recomendada)**
```go
// Crear procesos temporales para benchmark
type BenchmarkEngine struct {
    cmd    *exec.Cmd
    stdin  *bufio.Writer
    stdout *bufio.Reader
    mutex  sync.Mutex
}

func NewBenchmarkEngine(binaryPath string) (*BenchmarkEngine, error) {
    // Similar a EngineProcess pero para benchmark temporal
}

func runBenchmark(commitA, commitB string, games int, client *Client) {
    // 1. Compilar commit A a /tmp/motor_a
    // 2. Compilar commit B a /tmp/motor_b
    // 3. Crear BenchmarkEngine para cada uno
    // 4. Jugar partidas usando match.py lógica en Go
    // 5. Enviar progreso a cliente WebSocket
    // 6. Calcular Elo al final
}
```

**Solución 2: Reutilizar match.py (Más simple)**
```go
func runBenchmark(commitA, commitB string, games int, client *Client) {
    // Ejecutar: python3 match.py --engine1 /tmp/motor_a --engine2 /tmp/motor_b
    // Parsear stdout y enviar progreso por WebSocket
    // Calcular Elo al final
}
```

---

#### 2. Frontend - benchmark.html + benchmark.js

**Nuevos archivos:**
- `static/benchmark.html` - Página completa
- `static/js/benchmark.js` - Lógica JavaScript

**JavaScript estructura:**

```javascript
// benchmark.js
class BenchmarkController {
    constructor() {
        this.chessWS = chessWS; // Usar instancia global
        this.currentGames = 0;
        this.totalGames = 0;
        this.results = { wins: 0, losses: 0, draws: 0 };
    }
    
    init() {
        // 1. Pedir lista de commits al servidor
        this.chessWS.send({ type: 'get_commits' });
        
        // 2. Registrar handlers
        this.chessWS.on('commits_list', (payload) => {
            this.populateCommitDropdowns(payload);
        });
        
        this.chessWS.on('benchmark_started', (payload) => {
            this.showProgressPanel();
            this.totalGames = payload.games;
        });
        
        this.chessWS.on('benchmark_progress', (payload) => {
            // Actualizar progress bar y stats
            this.updateProgress(payload);
        });
        
        this.chessWS.on('benchmark_complete', (payload) => {
            // Mostrar resultados finales
            this.showResults(payload);
        });
    }
    
    startBenchmark() {
        const config = {
            type: 'start_benchmark',
            payload: {
                commit_a: document.getElementById('commit-a').value,
                commit_b: document.getElementById('commit-b').value,
                games: parseInt(document.getElementById('num-games').value),
                time_ms: 1000,
                keep_builds: false
            }
        };
        
        this.chessWS.send(config);
    }
}

// Inicializar
const benchmark = new BenchmarkController();
benchmark.init();
```

---

### Flujo de Usuario

```
1. Usuario va a /benchmark.html
        ↓
2. Frontend pide lista de commits al servidor
   → WebSocket: {type: "get_commits"}
        ↓
3. Servidor ejecuta: git log --oneline -20
   → Retorna: [{hash: "abc123", message: "feat: algo"}, ...]
        ↓
4. Frontend llena dropdowns:
   Commit A: [HEAD ▼]
   Commit B: [HEAD~1 ▼, HEAD~2 ▼, ...]
        ↓
5. Usuario selecciona A y B, pone "50 partidas"
   Clickea "Iniciar Benchmark"
        ↓
6. Frontend envía:
   {type: "start_benchmark", payload: {commit_a: "HEAD", commit_b: "HEAD~5", games: 50}}
        ↓
7. Servidor (en goroutine):
   a) git stash (si hay cambios)
   b) Compila commit A → /tmp/motor_a
   c) Compila commit B → /tmp/motor_b
   d) Juega partidas, envía progreso por WebSocket
   e) Calcula Elo
        ↓
8. Frontend muestra:
   - Progress bar: [████████░░] 32/50 partidas
   - Stats: A: 18W | 4D | B: 10W
   - Log: "Partida 32: A gana (45 movs)"
        ↓
9. Al terminar:
   - Muestra Elo: +127.4 ± 45.2
   - Intervalo: [82.2, 172.6]
   - Conclusión: ✅ Significativamente mejor
        ↓
10. Servidor limpia:
    - rm /tmp/motor_a /tmp/motor_b
    - git stash pop (si lo había guardado)
```

---

### Message Types Nuevos

| Dirección | Type | Payload | Descripción |
|-----------|------|---------|-------------|
| F→B | `get_commits` | - | Pedir lista de commits |
| B→F | `commits_list` | `[{hash, message, date}]` | Lista de commits |
| F→B | `start_benchmark` | `{commit_a, commit_b, games, time_ms}` | Iniciar benchmark |
| B→F | `benchmark_started` | `{games, commit_a, commit_b}` | Confirmación inicio |
| B→F | `benchmark_progress` | `{game_num, total, result, moves_count}` | Progreso en tiempo real |
| B→F | `benchmark_complete` | `{wins, losses, draws, elo, error_margin, ci_lower, ci_upper, los}` | Resultados finales |
| B→F | `benchmark_error` | `{message}` | Error durante benchmark |

---

### Complejidad de Implementación

**Backend (Go):**
- `get_commits` handler: ✅ Fácil (ejecutar git log, parsear)
- `start_benchmark` handler: ⚠️ Medio (gestionar 2 procesos, enviar progreso)
- Goroutine para no bloquear: ✅ Fácil
- Integración con match.py o reimplementar en Go: ⚠️ Medio

**Frontend (HTML/JS):**
- Nueva página benchmark.html: ✅ Fácil (copiar estructura debug.html)
- Dropdowns dinámicos: ✅ Fácil
- Progress bar: ✅ Fácil
- Update en tiempo real: ✅ Fácil (WebSocket handlers)
- Mostrar resultados Elo: ✅ Fácil

**Tiempo estimado:**
- Backend: 3-4 horas
- Frontend: 2-3 horas
- Testing: 1-2 horas
- **Total: ~6-8 horas de trabajo**

---

### Recomendación de Implementación

**Fase 1: Backend básico (3 horas)**
1. Agregar `get_commits` message type
2. Crear estructura `BenchmarkEngine` (similar a EngineProcess pero temporal)
3. Agregar `start_benchmark` que ejecute match.py como proceso externo
4. Parsear stdout de match.py y enviar progreso por WebSocket

**Fase 2: Frontend (2 horas)**
1. Crear `benchmark.html` basado en `debug.html`
2. Crear `benchmark.js` con dropdowns y progress
3. Conectar con WebSocket handlers nuevos

**Fase 3: Testing (1 hora)**
1. Probar con HEAD vs HEAD~1, 10 partidas
2. Verificar que Elo se calcula correctamente
3. Verificar cleanup de /tmp/

---

### Alternativa más simple (MVP)

Si querés algo rápido sin tocar el backend:

**Solución:** Agregar botón en debug.html que ejecute gitbench.py vía HTTP endpoint

```go
// server.go - Agregar endpoint HTTP (no WebSocket)
http.HandleFunc("/api/benchmark", func(w http.ResponseWriter, r *http.Request) {
    // Ejecutar: python3 benchmark/gitbench.py --vs-commit HEAD~1 --games 10
    // Stream output al cliente
})
```

**Ventaja:** No toca WebSocket, no bloquea nada.
**Desventaja:** No hay progreso en tiempo real, esperás hasta el final.

---

## Conclusión

**Arquitectura es segura:**
- gitbench.py ya funciona y prueba que el concepto funciona
- Go server puede extenderse sin romper nada existente
- Frontend WebSocket puede manejar mensajes nuevos fácilmente

**Mejor opción:** Página nueva `benchmark.html` con backend que ejecute match.py como proceso separado (no afecta el motor principal singleton).

¿Querés que implementemos esto? Te sugiero hacerlo en fases:
1. Primero backend básico (get_commits + start_benchmark simple)
2. Después frontend básico (página con dropdowns y botón)
3. Después progreso en tiempo real
4. Finalmente resultados con Elo
