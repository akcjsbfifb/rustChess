// Benchmark simple - solo muestra salida de Python en la web

class BenchmarkUI {
    constructor() {
        this.terminal = document.getElementById('terminal-output');
        this.statusBadge = document.getElementById('benchmark-status');
        this.isRunning = false;
        this.init();
    }

    init() {
        // Conectar WebSocket (igual que board.js)
        try {
            chessWS.connect();
        } catch (e) {
            console.error('[Benchmark] WebSocket connect error:', e);
        }

        // Cargar commits disponibles al inicio (HTTP, independiente del WebSocket)
        this.loadCommits();

        // Botón ejecutar
        document.getElementById('btn-run-benchmark').addEventListener('click', () => {
            this.runBenchmark();
        });

        // Botón limpiar
        document.getElementById('btn-clear-terminal').addEventListener('click', () => {
            this.terminal.innerHTML = '';
        });

        // Handler para mensajes WebSocket
        if (typeof chessWS !== 'undefined') {
            chessWS.on('benchmark_output', (payload) => {
                this.appendOutput(payload.line, payload.type);
            });

            chessWS.on('benchmark_complete', (payload) => {
                this.isRunning = false;
                this.updateStatus('Completado', 'success');
                this.appendOutput('\n✅ Benchmark completado', 'result');
            });

            chessWS.on('benchmark_error', (payload) => {
                this.isRunning = false;
                this.updateStatus('Error', 'error');
                this.appendOutput(`\n❌ Error: ${payload.message}`, 'error');
            });
        }
    }

    loadCommits() {
        const selectA = document.getElementById('commit-a');
        const selectB = document.getElementById('commit-b');
        
        // Limpiar opciones existentes (excepto la primera)
        while (selectA.children.length > 1) selectA.removeChild(selectA.lastChild);
        while (selectB.children.length > 1) selectB.removeChild(selectB.lastChild);
        
        // Mensaje de carga
        const loadingA = document.createElement('option');
        loadingA.textContent = 'Cargando commits...';
        selectA.appendChild(loadingA);
        
        const loadingB = document.createElement('option');
        loadingB.textContent = 'Cargando commits...';
        selectB.appendChild(loadingB);

        // HTTP GET simple - funciona con Tailscale y es más confiable que WebSocket
        console.log('[Benchmark] Fetching commits via HTTP...');
        fetch('/api/commits')
            .then(response => {
                if (!response.ok) {
                    if (response.status === 503) {
                        throw new Error('Servidor ocupado, intentá más tarde');
                    }
                    throw new Error(`Error del servidor: ${response.status}`);
                }
                return response.json();
            })
            .then(commits => {
                if (!Array.isArray(commits)) {
                    throw new Error('Respuesta inválida del servidor');
                }
                if (commits.length === 0) {
                    throw new Error('No se encontraron commits');
                }
                console.log('[Benchmark] Received', commits.length, 'commits via HTTP');
                this.populateDropdowns(commits);
            })
            .catch(error => {
                console.error('[Benchmark] Failed to load commits:', error);
                // No mostrar error en terminal, solo usar valores por defecto
                this.loadDefaultCommits();
            });
    }
    
    loadDefaultCommits() {
        // Fallback si el servidor no responde
        const selectA = document.getElementById('commit-a');
        const selectB = document.getElementById('commit-b');
        
        const defaultCommits = [
            { value: 'HEAD', label: 'HEAD (actual)' },
            { value: 'HEAD~1', label: 'HEAD~1 (1 commit atrás)' },
            { value: 'HEAD~3', label: 'HEAD~3 (3 commits atrás)' },
            { value: 'HEAD~5', label: 'HEAD~5 (5 commits atrás)' },
        ];
        
        defaultCommits.forEach(commit => {
            const optionA = document.createElement('option');
            optionA.value = commit.value;
            optionA.textContent = commit.label;
            selectA.appendChild(optionA);
            
            const optionB = document.createElement('option');
            optionB.value = commit.value;
            optionB.textContent = commit.label;
            selectB.appendChild(optionB);
        });
    }

    populateDropdowns(commits) {
        const selectA = document.getElementById('commit-a');
        const selectB = document.getElementById('commit-b');

        // Limpiar opciones existentes (excepto primera)
        while (selectA.children.length > 1) selectA.removeChild(selectA.lastChild);
        while (selectB.children.length > 1) selectB.removeChild(selectB.lastChild);

        commits.forEach(commit => {
            const optionA = document.createElement('option');
            optionA.value = commit.hash;
            optionA.textContent = `${commit.hash} - ${commit.message}`;
            selectA.appendChild(optionA);

            const optionB = document.createElement('option');
            optionB.value = commit.hash;
            optionB.textContent = `${commit.hash} - ${commit.message}`;
            selectB.appendChild(optionB);
        });
    }

    runBenchmark() {
        if (this.isRunning) {
            alert('Ya hay un benchmark en ejecución');
            return;
        }

        const commitA = document.getElementById('commit-a').value;
        const commitB = document.getElementById('commit-b').value;
        const games = parseInt(document.getElementById('num-games').value);

        // Validación de inputs
        if (!commitA || !commitB) {
            alert('Seleccioná ambos commits para comparar');
            return;
        }

        if (commitA === commitB) {
            alert('Los dos commits deben ser diferentes');
            return;
        }

        if (!games || games < 1 || games > 200) {
            alert('Número de partidas inválido (1-200)');
            return;
        }

        // Validar formato de hash (solo hex chars, 7-40 chars)
        const isValidHash = (hash) => /^[a-f0-9]{7,40}$/i.test(hash);
        if (!isValidHash(commitA) || !isValidHash(commitB)) {
            alert('Formato de commit inválido');
            return;
        }

        this.isRunning = true;
        this.terminal.innerHTML = '';
        this.updateStatus('Ejecutando...', 'running');

        // Enviar comando al servidor con manejo de error
        try {
            chessWS.send({
                type: 'run_benchmark',
                payload: {
                    commit_a: commitA,
                    commit_b: commitB,
                    games: games
                }
            });

            this.appendOutput(`Iniciando benchmark...\n`, 'info');
            this.appendOutput(`Motor A: ${commitA}\n`, 'info');
            this.appendOutput(`Motor B: ${commitB}\n`, 'info');
            this.appendOutput(`Partidas: ${games}\n`, 'info');
            this.appendOutput(`\n`, 'info');
        } catch (e) {
            this.appendOutput(`❌ Error al enviar comando: ${e.message}`, 'error');
            this.isRunning = false;
            this.updateStatus('Error', 'error');
        }
    }

    appendOutput(text, type = 'info') {
        const line = document.createElement('div');
        line.className = `log-${type}`;
        line.textContent = text;
        this.terminal.appendChild(line);
        this.terminal.scrollTop = this.terminal.scrollHeight;
    }

    updateStatus(text, type) {
        this.statusBadge.textContent = text;
        this.statusBadge.className = 'px-3 py-1 rounded-full text-xs font-bold text-white';

        switch (type) {
            case 'running':
                this.statusBadge.classList.add('bg-[#e6912c]');
                break;
            case 'success':
                this.statusBadge.classList.add('bg-[#81b64c]');
                break;
            case 'error':
                this.statusBadge.classList.add('bg-[#f44336]');
                break;
            default:
                this.statusBadge.classList.add('bg-[#4d4d4d]');
        }
    }
}

// Inicializar cuando el DOM esté listo
document.addEventListener('DOMContentLoaded', () => {
    window.benchmarkUI = new BenchmarkUI();
});
