// Benchmark simple - solo muestra salida de Python en la web

class BenchmarkUI {
    constructor() {
        this.terminal = document.getElementById('terminal-output');
        this.statusBadge = document.getElementById('benchmark-status');
        this.isRunning = false;
        this.init();
    }

    init() {
        // Cargar commits disponibles al inicio
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
        // En una versión simple, pedimos los commits al servidor
        // Por ahora, usamos valores hardcodeados comunes
        const selectB = document.getElementById('commit-b');
        const commonCommits = [
            { value: 'HEAD~1', label: 'HEAD~1 (1 commit atrás)' },
            { value: 'HEAD~3', label: 'HEAD~3 (3 commits atrás)' },
            { value: 'HEAD~5', label: 'HEAD~5 (5 commits atrás)' },
            { value: 'HEAD~10', label: 'HEAD~10 (10 commits atrás)' },
        ];

        commonCommits.forEach(commit => {
            const option = document.createElement('option');
            option.value = commit.value;
            option.textContent = commit.label;
            selectB.appendChild(option);
        });

        // Si el servidor soporta get_commits, lo usamos
        if (typeof chessWS !== 'undefined' && chessWS.isConnected) {
            chessWS.send({ type: 'get_commits' });
            chessWS.on('commits_list', (payload) => {
                // Actualizar dropdowns con commits reales
                this.populateDropdowns(payload);
            });
        }
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

        if (!commitB) {
            alert('Seleccioná el commit B para comparar');
            return;
        }

        if (commitA === commitB) {
            alert('Los dos commits deben ser diferentes');
            return;
        }

        this.isRunning = true;
        this.terminal.innerHTML = '';
        this.updateStatus('Ejecutando...', 'running');

        // Enviar comando al servidor
        if (typeof chessWS !== 'undefined') {
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
        } else {
            this.appendOutput('❌ WebSocket no conectado', 'error');
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
