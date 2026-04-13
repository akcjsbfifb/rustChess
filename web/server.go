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

// runBenchmark ejecuta gitbench.py y envía la salida al cliente
func (c *Client) runBenchmark(commitA, commitB string, games int) {
	log.Printf("[BENCHMARK] Starting: %s vs %s (%d games)", commitA, commitB, games)

	// Preparar comando
	var cmd *exec.Cmd
	if commitA == "HEAD" {
		// Usar gitbench.py para HEAD vs commitB
		cmd = exec.Command("python3", "../benchmark/gitbench.py",
			"--vs-commit", commitB,
			"--games", fmt.Sprintf("%d", games),
			"--openings", "../benchmark/openings.epd")
	} else {
		// Usar match.py directamente si ambos commits son específicos
		// (requiere compilar primero, simplificamos usando gitbench)
		c.sendBenchmarkLine(fmt.Sprintf("Modo avanzado: compilando commits %s y %s...", commitA, commitB), "info")
		c.sendBenchmarkLine("Usando comparación HEAD vs commit", "info")
		cmd = exec.Command("python3", "../benchmark/gitbench.py",
			"--vs-commit", commitB,
			"--games", fmt.Sprintf("%d", games),
			"--openings", "../benchmark/openings.epd")
	}

	cmd.Dir = "."

	// Capturar stdout y stderr
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		c.sendBenchmarkError(fmt.Sprintf("Failed to create stdout pipe: %v", err))
		return
	}

	stderr, err := cmd.StderrPipe()
	if err != nil {
		c.sendBenchmarkError(fmt.Sprintf("Failed to create stderr pipe: %v", err))
		return
	}

	// Iniciar comando
	if err := cmd.Start(); err != nil {
		c.sendBenchmarkError(fmt.Sprintf("Failed to start benchmark: %v", err))
		return
	}

	// Leer stdout en goroutine
	go func() {
		scanner := bufio.NewScanner(stdout)
		for scanner.Scan() {
			line := scanner.Text()
			c.sendBenchmarkLine(line, "info")
		}
	}()

	// Leer stderr en goroutine
	go func() {
		scanner := bufio.NewScanner(stderr)
		for scanner.Scan() {
			line := scanner.Text()
			c.sendBenchmarkLine(line, "error")
		}
	}()

	// Esperar a que termine
	if err := cmd.Wait(); err != nil {
		log.Printf("[BENCHMARK] Process finished with error: %v", err)
		// No enviar error si ya recibimos output
	}

	// Enviar evento de completado
	c.conn.WriteJSON(Response{
		Type:    "benchmark_complete",
		Payload: map[string]string{"status": "done"},
	})

	log.Printf("[BENCHMARK] Completed: %s vs %s", commitA, commitB)
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
