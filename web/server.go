package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Permitir conexiones desde cualquier origen (localhost)
	},
}

// Engine singleton - compartido entre todos los clientes
var (
	sharedEngine     *EngineProcess
	sharedEngineMux  sync.Mutex
	sharedEngineOnce sync.Once
)

// GetSharedEngine obtiene la instancia compartida del engine (singleton)
func GetSharedEngine() (*EngineProcess, error) {
	var err error
	sharedEngineOnce.Do(func() {
		sharedEngine, err = NewEngineProcess()
	})
	return sharedEngine, err
}

// EngineProcess encapsula el proceso del engine de ajedrez
type EngineProcess struct {
	cmd    *exec.Cmd
	stdin  *bufio.Writer
	stdout *bufio.Reader
	mutex  sync.Mutex
}

// NewEngineProcess inicia el proceso del engine Rust
func NewEngineProcess() (*EngineProcess, error) {
	// Buscar el binario del engine
	enginePath := "./target/release/rust_chess"

	// Intentar rutas alternativas si no existe
	if _, err := os.Stat(enginePath); os.IsNotExist(err) {
		// Intentar desde el directorio padre
		enginePath = "../target/release/rust_chess"
		if _, err := os.Stat(enginePath); os.IsNotExist(err) {
			return nil, fmt.Errorf("engine binary not found")
		}
	}

	cmd := exec.Command(enginePath)

	stdin, err := cmd.StdinPipe()
	if err != nil {
		return nil, fmt.Errorf("failed to get stdin pipe: %v", err)
	}

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return nil, fmt.Errorf("failed to get stdout pipe: %v", err)
	}

	stderr, err := cmd.StderrPipe()
	if err != nil {
		return nil, fmt.Errorf("failed to get stderr pipe: %v", err)
	}

	if err := cmd.Start(); err != nil {
		return nil, fmt.Errorf("failed to start engine: %v", err)
	}

	// Goroutine para loggear stderr
	go func() {
		scanner := bufio.NewScanner(stderr)
		for scanner.Scan() {
			log.Printf("[Engine stderr] %s", scanner.Text())
		}
	}()

	return &EngineProcess{
		cmd:    cmd,
		stdin:  bufio.NewWriter(stdin),
		stdout: bufio.NewReader(stdout),
	}, nil
}

// SendCommand envía un comando al engine y devuelve la respuesta
func (e *EngineProcess) SendCommand(command string) (string, error) {
	e.mutex.Lock()
	defer e.mutex.Unlock()

	// Log comando enviado
	log.Printf("[STDIN → Engine] %s", command)

	// Enviar comando
	if _, err := e.stdin.WriteString(command + "\n"); err != nil {
		return "", fmt.Errorf("failed to write command: %v", err)
	}
	if err := e.stdin.Flush(); err != nil {
		return "", fmt.Errorf("failed to flush command: %v", err)
	}

	// Leer respuesta (línea JSON)
	line, err := e.stdout.ReadString('\n')
	if err != nil {
		return "", fmt.Errorf("failed to read response: %v", err)
	}

	line = strings.TrimSpace(line)

	// Log respuesta recibida
	if len(line) > 100 {
		log.Printf("[STDOUT ← Engine] %s... (truncado)", line[:100])
	} else {
		log.Printf("[STDOUT ← Engine] %s", line)
	}

	return line, nil
}

// Stop termina el proceso del engine
func (e *EngineProcess) Stop() error {
	if e.cmd != nil && e.cmd.Process != nil {
		return e.cmd.Process.Kill()
	}
	return nil
}

// Client representa una conexión WebSocket
type Client struct {
	conn   *websocket.Conn
	engine *EngineProcess
}

// Message representa un mensaje WebSocket
type Message struct {
	Type    string          `json:"type"`
	Payload json.RawMessage `json:"payload,omitempty"`
}

// Response representa una respuesta al cliente
type Response struct {
	Type    string      `json:"type"`
	Payload interface{} `json:"payload"`
}

func main() {
	// Configurar logging
	log.SetFlags(log.Ltime | log.Lmicroseconds)

	// Obtener directorio actual
	ex, err := os.Executable()
	if err != nil {
		log.Fatal(err)
	}
	staticDir := filepath.Join(filepath.Dir(ex), "static")

	// Si estamos en desarrollo, usar ruta relativa
	if _, err := os.Stat(staticDir); os.IsNotExist(err) {
		staticDir = "./static"
	}

	// Servir archivos estáticos
	fs := http.FileServer(http.Dir(staticDir))
	http.Handle("/", fs)

	// Endpoint WebSocket
	http.HandleFunc("/ws", handleWebSocket)

	log.Println("Server starting on http://0.0.0.0:8080")
	log.Println("Open http://<TAILSCALE_IP>:8080 in your browser")

	if err := http.ListenAndServe("0.0.0.0:8080", nil); err != nil {
		log.Fatal("Server error:", err)
	}
}

func handleWebSocket(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("WebSocket upgrade error: %v", err)
		return
	}
	defer conn.Close()

	log.Println("Client connected")

	// Obtener engine compartido (singleton)
	engine, err := GetSharedEngine()
	if err != nil {
		log.Printf("Failed to get shared engine: %v", err)
		conn.WriteJSON(Response{
			Type:    "error",
			Payload: map[string]string{"message": "Failed to start chess engine"},
		})
		return
	}

	// Enviar estado actual (puede ser una partida en curso)
	state, err := engine.SendCommand("state")
	if err != nil {
		log.Printf("Get state error: %v", err)
	} else {
		conn.WriteJSON(Response{
			Type:    "board_state",
			Payload: json.RawMessage(state),
		})
	}

	client := &Client{conn: conn, engine: engine}

	// Loop principal de mensajes
	for {
		var msg Message
		if err := conn.ReadJSON(&msg); err != nil {
			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
				log.Printf("WebSocket error: %v", err)
			}
			break
		}

		if err := client.handleMessage(msg); err != nil {
			log.Printf("Message handler error: %v", err)
			conn.WriteJSON(Response{
				Type:    "error",
				Payload: map[string]string{"message": err.Error()},
			})
		}
	}

	log.Println("Client disconnected")
}

func (c *Client) handleMessage(msg Message) error {
	switch msg.Type {
	case "init":
		var payload struct {
			FEN string `json:"fen"`
		}
		if err := json.Unmarshal(msg.Payload, &payload); err != nil {
			return err
		}

		cmd := "position startpos"
		if payload.FEN != "" {
			cmd = fmt.Sprintf("position fen %s", payload.FEN)
		}

		response, err := c.engine.SendCommand(cmd)
		if err != nil {
			return err
		}

		return c.sendBoardState(response)

	case "get_moves":
		response, err := c.engine.SendCommand("moves")
		if err != nil {
			return err
		}

		c.conn.WriteJSON(Response{
			Type:    "legal_moves",
			Payload: json.RawMessage(response),
		})
		return nil

	case "make_move":
		var payload struct {
			Move string `json:"move"`
		}
		if err := json.Unmarshal(msg.Payload, &payload); err != nil {
			return err
		}

		cmd := fmt.Sprintf("move %s", payload.Move)
		response, err := c.engine.SendCommand(cmd)
		if err != nil {
			return err
		}

		return c.sendBoardState(response)

	case "undo":
		response, err := c.engine.SendCommand("undo")
		if err != nil {
			return err
		}

		return c.sendBoardState(response)

	case "engine_go":
		var payload struct {
			Depth int `json:"depth"`
		}
		// Default depth 4 if not specified
		depth := 4
		if err := json.Unmarshal(msg.Payload, &payload); err == nil && payload.Depth > 0 {
			depth = payload.Depth
		}

		log.Printf("[ENGINE] Starting search with depth %d...", depth)
		startTime := time.Now()

		cmd := fmt.Sprintf("go %d", depth)
		response, err := c.engine.SendCommand(cmd)
		if err != nil {
			return err
		}

		elapsed := time.Since(startTime)

		// Parse response to show thinking info
		var result map[string]interface{}
		if err := json.Unmarshal([]byte(response), &result); err == nil {
			bestMove, _ := result["best_move"].(string)
			eval, _ := result["eval"].(float64)
			nodes, _ := result["nodes"].(float64)
			engineDepth, _ := result["depth"].(float64)
			timeMs, _ := result["time_ms"].(float64)

			nps := float64(0)
			if timeMs > 0 {
				nps = (nodes / timeMs) * 1000
			}

			log.Printf("[ENGINE] Search complete!")
			log.Printf("[ENGINE] Best move: %s | Eval: %.0f | Depth: %.0f", bestMove, eval, engineDepth)
			log.Printf("[ENGINE] Nodes: %.0f | Time: %.0f ms | NPS: %.0f", nodes, timeMs, nps)
			log.Printf("[ENGINE] Total response time: %v", elapsed)
		}

		c.conn.WriteJSON(Response{
			Type:    "best_move",
			Payload: json.RawMessage(response),
		})

		// Actualizar estado después de que el engine juegue
		if err := json.Unmarshal([]byte(response), &result); err == nil {
			if bestMove, ok := result["best_move"].(string); ok && bestMove != "" {
				c.engine.SendCommand(fmt.Sprintf("move %s", bestMove))
				state, _ := c.engine.SendCommand("state")
				c.conn.WriteJSON(Response{
					Type:    "board_state",
					Payload: json.RawMessage(state),
				})
			}
		}
		return nil

	case "perft":
		var payload struct {
			Depth int `json:"depth"`
		}
		if err := json.Unmarshal(msg.Payload, &payload); err != nil {
			return err
		}

		cmd := fmt.Sprintf("perft %d", payload.Depth)
		response, err := c.engine.SendCommand(cmd)
		if err != nil {
			return err
		}

		c.conn.WriteJSON(Response{
			Type:    "perft_result",
			Payload: json.RawMessage(response),
		})
		return nil

	case "get_state":
		response, err := c.engine.SendCommand("state")
		if err != nil {
			return err
		}

		c.conn.WriteJSON(Response{
			Type:    "board_state",
			Payload: json.RawMessage(response),
		})
		return nil

	case "get_commits":
		// Obtener últimos 20 commits
		commits, err := getGitCommits(20)
		if err != nil {
			return err
		}

		c.conn.WriteJSON(Response{
			Type:    "commits_list",
			Payload: commits,
		})
		return nil

	case "run_benchmark":
		var payload struct {
			CommitA string `json:"commit_a"`
			CommitB string `json:"commit_b"`
			Games   int    `json:"games"`
		}
		if err := json.Unmarshal(msg.Payload, &payload); err != nil {
			return err
		}

		// Ejecutar benchmark en goroutine (no bloquear)
		go c.runBenchmark(payload.CommitA, payload.CommitB, payload.Games)

		c.conn.WriteJSON(Response{
			Type: "benchmark_started",
			Payload: map[string]interface{}{
				"commit_a": payload.CommitA,
				"commit_b": payload.CommitB,
				"games":    payload.Games,
			},
		})
		return nil

	default:
		return fmt.Errorf("unknown message type: %s", msg.Type)
	}
}

func (c *Client) sendBoardState(cmdResponse string) error {
	// Obtener estado actual del board
	state, err := c.engine.SendCommand("state")
	if err != nil {
		return err
	}

	c.conn.WriteJSON(Response{
		Type:    "command_response",
		Payload: json.RawMessage(cmdResponse),
	})

	c.conn.WriteJSON(Response{
		Type:    "board_state",
		Payload: json.RawMessage(state),
	})
	return nil
}

// CommitInfo representa un commit de git
type CommitInfo struct {
	Hash    string `json:"hash"`
	Message string `json:"message"`
	Date    string `json:"date"`
}

// getGitCommits obtiene los últimos N commits del repositorio
func getGitCommits(n int) ([]CommitInfo, error) {
	cmd := exec.Command("git", "log", "--oneline", "-"+fmt.Sprintf("%d", n), "--pretty=format:%h|%s|%ar")
	cmd.Dir = ".." // Ejecutar desde directorio padre (donde está el repo)

	output, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("failed to get git commits: %v", err)
	}

	lines := strings.Split(string(output), "\n")
	var commits []CommitInfo

	for _, line := range lines {
		if line == "" {
			continue
		}
		parts := strings.SplitN(line, "|", 3)
		if len(parts) >= 2 {
			date := ""
			if len(parts) >= 3 {
				date = parts[2]
			}
			commits = append(commits, CommitInfo{
				Hash:    parts[0],
				Message: parts[1],
				Date:    date,
			})
		}
	}

	return commits, nil
}

// runBenchmark compila dos commits cualquiera y ejecuta match entre ellos
func (c *Client) runBenchmark(commitA, commitB string, games int) {
	log.Printf("[BENCHMARK] ============================================")
	log.Printf("[BENCHMARK] Starting comparison: %s vs %s (%d games)", commitA, commitB, games)
	log.Printf("[BENCHMARK] ============================================")

	// Verificar commits
	log.Printf("[BENCHMARK] Step 1: Verifying commits...")
	if err := verifyCommit(commitA); err != nil {
		log.Printf("[BENCHMARK] ERROR: Commit A invalid: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Commit A invalid: %v", err))
		return
	}
	if err := verifyCommit(commitB); err != nil {
		log.Printf("[BENCHMARK] ERROR: Commit B invalid: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Commit B invalid: %v", err))
		return
	}

	// Guardar referencia actual
	log.Printf("[BENCHMARK] Step 2: Saving current state...")
	originalRef, err := getCurrentGitRef()
	if err != nil {
		log.Printf("[BENCHMARK] ERROR: Cannot get current ref: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Cannot get current ref: %v", err))
		return
	}
	log.Printf("[BENCHMARK] Current ref: %s", originalRef)

	// Verificar si hay cambios sin commitear
	log.Printf("[BENCHMARK] Step 3: Checking for uncommitted changes...")
	hasChanges := hasUncommittedChanges()
	if hasChanges {
		log.Printf("[BENCHMARK] Stashing uncommitted changes...")
		c.sendBenchmarkLine("📦 Guardando cambios actuales...", "info")
		if err := stashChanges(); err != nil {
			log.Printf("[BENCHMARK] ERROR: Failed to stash: %v", err)
			c.sendBenchmarkError(fmt.Sprintf("Failed to stash changes: %v", err))
			return
		}
	}

	// Crear directorio temporal
	tempDir, err := os.MkdirTemp("", "rust_chess_bench_*")
	if err != nil {
		log.Printf("[BENCHMARK] ERROR: Cannot create temp dir: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Cannot create temp dir: %v", err))
		return
	}
	defer os.RemoveAll(tempDir)
	log.Printf("[BENCHMARK] Step 4: Created temp dir: %s", tempDir)

	motorA := filepath.Join(tempDir, "motor_a")
	motorB := filepath.Join(tempDir, "motor_b")

	// Compilar Motor A
	log.Printf("[BENCHMARK] Step 5: Compiling Motor A (%s)...", commitA)
	c.sendBenchmarkLine(fmt.Sprintf("🔨 Compilando Motor A (%s)...", commitA), "info")
	if err := compileCommit(commitA, motorA); err != nil {
		log.Printf("[BENCHMARK] ERROR: Failed to compile Motor A: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Failed to compile Motor A: %v", err))
		returnToOriginalRef(originalRef)
		return
	}
	c.sendBenchmarkLine(fmt.Sprintf("✅ Motor A compilado: %s", motorA), "info")

	// Compilar Motor B
	log.Printf("[BENCHMARK] Step 6: Compiling Motor B (%s)...", commitB)
	c.sendBenchmarkLine(fmt.Sprintf("🔨 Compilando Motor B (%s)...", commitB), "info")
	if err := compileCommit(commitB, motorB); err != nil {
		log.Printf("[BENCHMARK] ERROR: Failed to compile Motor B: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Failed to compile Motor B: %v", err))
		returnToOriginalRef(originalRef)
		return
	}
	c.sendBenchmarkLine(fmt.Sprintf("✅ Motor B compilado: %s", motorB), "info")

	// Volver a la referencia original
	log.Printf("[BENCHMARK] Step 7: Returning to original ref: %s", originalRef)
	if err := returnToOriginalRef(originalRef); err != nil {
		log.Printf("[BENCHMARK] WARNING: Failed to return to original ref: %v", err)
	}

	// Restaurar stash si existía
	if hasChanges {
		log.Printf("[BENCHMARK] Restoring stashed changes...")
		if err := popStash(); err != nil {
			log.Printf("[BENCHMARK] WARNING: Failed to pop stash: %v", err)
		}
	}

	// Ejecutar match.py
	log.Printf("[BENCHMARK] Step 8: Running match.py (%d games)...", games)
	c.sendBenchmarkLine(fmt.Sprintf("🏁 Iniciando match: %d partidas", games), "info")
	c.sendBenchmarkLine(fmt.Sprintf("   Motor A: %s", commitA), "info")
	c.sendBenchmarkLine(fmt.Sprintf("   Motor B: %s", commitB), "info")
	c.sendBenchmarkLine("", "info")

	openingsPath := filepath.Join("..", "benchmark", "openings.epd")
	cmd := exec.Command("python3", filepath.Join("..", "benchmark", "match.py"),
		"--engine1", motorA,
		"--engine2", motorB,
		"--games", fmt.Sprintf("%d", games),
		"--openings", openingsPath,
	)

	cmd.Dir = "."

	// Capturar stdout y stderr
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		log.Printf("[BENCHMARK] ERROR: Failed to create stdout pipe: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Failed to create stdout pipe: %v", err))
		return
	}

	stderr, err := cmd.StderrPipe()
	if err != nil {
		log.Printf("[BENCHMARK] ERROR: Failed to create stderr pipe: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Failed to create stderr pipe: %v", err))
		return
	}

	// Iniciar comando
	log.Printf("[BENCHMARK] Starting match.py process...")
	if err := cmd.Start(); err != nil {
		log.Printf("[BENCHMARK] ERROR: Failed to start match: %v", err)
		c.sendBenchmarkError(fmt.Sprintf("Failed to start match: %v", err))
		return
	}
	log.Printf("[BENCHMARK] match.py PID: %d", cmd.Process.Pid)

	// Leer stdout en goroutine
	go func() {
		scanner := bufio.NewScanner(stdout)
		for scanner.Scan() {
			line := scanner.Text()
			log.Printf("[BENCHMARK stdout] %s", line)
			c.sendBenchmarkLine(line, "info")
		}
		if err := scanner.Err(); err != nil {
			log.Printf("[BENCHMARK stdout error] %v", err)
		}
	}()

	// Leer stderr en goroutine
	go func() {
		scanner := bufio.NewScanner(stderr)
		for scanner.Scan() {
			line := scanner.Text()
			log.Printf("[BENCHMARK stderr] %s", line)
			c.sendBenchmarkLine(line, "error")
		}
	}()

	// Esperar a que termine
	log.Printf("[BENCHMARK] Waiting for match.py to complete...")
	if err := cmd.Wait(); err != nil {
		log.Printf("[BENCHMARK] match.py finished with error: %v", err)
	} else {
		log.Printf("[BENCHMARK] match.py completed successfully")
	}

	// Enviar evento de completado
	c.conn.WriteJSON(Response{
		Type:    "benchmark_complete",
		Payload: map[string]string{"status": "done"},
	})

	log.Printf("[BENCHMARK] ============================================")
	log.Printf("[BENCHMARK] Comparison completed: %s vs %s", commitA, commitB)
	log.Printf("[BENCHMARK] ============================================")
}

// Funciones auxiliares para benchmark

func verifyCommit(hash string) error {
	cmd := exec.Command("git", "cat-file", "-t", hash)
	cmd.Dir = ".."
	output, err := cmd.Output()
	if err != nil {
		return fmt.Errorf("commit not found: %s", hash)
	}
	if strings.TrimSpace(string(output)) != "commit" {
		return fmt.Errorf("not a valid commit: %s", hash)
	}
	return nil
}

func getCurrentGitRef() (string, error) {
	cmd := exec.Command("git", "rev-parse", "--abbrev-ref", "HEAD")
	cmd.Dir = ".."
	output, err := cmd.Output()
	if err != nil {
		return "", err
	}
	ref := strings.TrimSpace(string(output))
	if ref == "HEAD" {
		// Estamos en detached HEAD, usar el hash
		cmd = exec.Command("git", "rev-parse", "HEAD")
		cmd.Dir = ".."
		output, err = cmd.Output()
		if err != nil {
			return "", err
		}
		ref = strings.TrimSpace(string(output))
	}
	return ref, nil
}

func hasUncommittedChanges() bool {
	cmd := exec.Command("git", "status", "--porcelain")
	cmd.Dir = ".."
	output, err := cmd.Output()
	if err != nil {
		return false
	}
	return len(strings.TrimSpace(string(output))) > 0
}

func stashChanges() error {
	cmd := exec.Command("git", "stash", "push", "-m", "benchmark-autostash")
	cmd.Dir = ".."
	return cmd.Run()
}

func popStash() error {
	cmd := exec.Command("git", "stash", "pop")
	cmd.Dir = ".."
	return cmd.Run()
}

func compileCommit(hash string, outputPath string) error {
	log.Printf("[COMPILE] Checking out %s...", hash)

	// Checkout al commit
	cmd := exec.Command("git", "checkout", hash)
	cmd.Dir = ".."
	if output, err := cmd.CombinedOutput(); err != nil {
		return fmt.Errorf("git checkout failed: %v - %s", err, string(output))
	}

	log.Printf("[COMPILE] Building %s...", hash)

	// Compilar
	cmd = exec.Command("cargo", "build", "--release")
	cmd.Dir = ".."
	if output, err := cmd.CombinedOutput(); err != nil {
		return fmt.Errorf("cargo build failed: %v - %s", err, string(output))
	}

	// Copiar binario
	src := "../target/release/rust_chess"
	if _, err := os.Stat(src); os.IsNotExist(err) {
		return fmt.Errorf("binary not found at %s", src)
	}

	if err := copyFile(src, outputPath); err != nil {
		return fmt.Errorf("copy failed: %v", err)
	}

	log.Printf("[COMPILE] %s compiled successfully to %s", hash, outputPath)
	return nil
}

func returnToOriginalRef(ref string) error {
	log.Printf("[GIT] Returning to %s...", ref)
	cmd := exec.Command("git", "checkout", ref)
	cmd.Dir = ".."
	if output, err := cmd.CombinedOutput(); err != nil {
		return fmt.Errorf("git checkout failed: %v - %s", err, string(output))
	}
	return nil
}

func copyFile(src, dst string) error {
	input, err := os.ReadFile(src)
	if err != nil {
		return err
	}
	return os.WriteFile(dst, input, 0755)
}

// sendBenchmarkLine envía una línea de output al cliente
func (c *Client) sendBenchmarkLine(line string, lineType string) {
	// Determinar tipo basado en contenido
	if strings.Contains(line, "✅") || strings.Contains(line, "✓") || strings.Contains(line, "CONCLUSIÓN: El motor A es SIGNIFICATIVAMENTE MEJOR") {
		lineType = "result"
	} else if strings.Contains(line, "❌") || strings.Contains(line, "Error") || strings.Contains(line, "Falla") {
		lineType = "error"
	} else if strings.Contains(line, "gana") || strings.Contains(line, "Empate") || strings.Contains(line, "movs") {
		lineType = "progress"
	} else if strings.Contains(line, "ELO") || strings.Contains(line, "Intervalo") || strings.Contains(line, "Victorias") {
		lineType = "result"
	}

	c.conn.WriteJSON(Response{
		Type: "benchmark_output",
		Payload: map[string]string{
			"line": line,
			"type": lineType,
		},
	})
}

// sendBenchmarkError envía un error al cliente
func (c *Client) sendBenchmarkError(message string) {
	c.conn.WriteJSON(Response{
		Type: "benchmark_error",
		Payload: map[string]string{
			"message": message,
		},
	})
}
