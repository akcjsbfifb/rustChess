class DebugPanel {
    constructor() {
        this.logContainer = document.getElementById('log-container');
        this.messageLog = [];
        this.maxLogEntries = 100;
        this.init();
    }
    
    init() {
        this.setupEventListeners();
        this.setupWebSocketHandlers();
        chessWS.connect();
        this.createBoardRep();
    }
    
    setupEventListeners() {
        document.getElementById('btn-run-perft').addEventListener('click', () => {
            const depth = parseInt(document.getElementById('perft-depth').value) || 4;
            document.getElementById('perft-results').innerHTML = '<pre>Calculando...</pre>';
            chessWS.send({ type: 'perft', payload: { depth } });
        });
        
        document.getElementById('btn-clear-log').addEventListener('click', () => {
            this.messageLog = [];
            this.logContainer.innerHTML = '';
        });
        
        // Listen for all chess messages
        window.addEventListener('chess-message', (e) => {
            this.logMessage(e.detail, 'received');
        });
        
        // Override send to log outgoing messages
        const originalSend = chessWS.send.bind(chessWS);
        chessWS.send = (msg) => {
            this.logMessage(msg, 'sent');
            originalSend(msg);
        };
    }
    
    setupWebSocketHandlers() {
        chessWS.on('board_state', (state) => {
            this.updateState(state);
        });
        
        chessWS.on('legal_moves', (data) => {
            this.updateMovesDetail(data);
        });
        
        chessWS.on('perft_result', (data) => {
            this.updatePerftResults(data);
        });
        
        chessWS.on('error', (data) => {
            this.logMessage(data, 'error');
        });
    }
    
    updateState(state) {
        document.getElementById('debug-fen').textContent = state.fen || '-';
        document.getElementById('debug-turn').textContent = state.turn || '-';
        document.getElementById('debug-castling').textContent = state.castling_rights || '-';
        document.getElementById('debug-enpassant').textContent = state.en_passant || 'None';
        document.getElementById('debug-wking').textContent = state.white_king || '-';
        document.getElementById('debug-bking').textContent = state.black_king || '-';
        
        this.updateBoardRep(state);
    }
    
    createBoardRep() {
        const container = document.getElementById('board-rep');
        container.innerHTML = '';
        
        // Tablero 10x12 - representación exacta del array interno del engine
        const grid = document.createElement('div');
        grid.className = 'inline-block font-mono text-xs';
        grid.id = 'debug-board-grid';
        
        // Header row (índices de columna 0-9)
        const headerRow = document.createElement('div');
        headerRow.className = 'flex';
        headerRow.innerHTML = '<div class="w-8 h-6 bg-[#404040] text-white font-bold flex items-center justify-center border border-[#555]"></div>' + 
            Array.from({length: 10}, (_, i) => `<div class="w-8 h-6 bg-[#404040] text-white font-bold flex items-center justify-center border border-[#555]">${i}</div>`).join('');
        grid.appendChild(headerRow);
        
        // Rows 0-11 (el tablero real del engine)
        for (let rank = 0; rank < 12; rank++) {
            const row = document.createElement('div');
            row.className = 'flex';
            
            // Row label (índice de fila)
            const rowLabel = document.createElement('div');
            rowLabel.className = 'w-8 h-6 bg-[#404040] text-white font-bold flex items-center justify-center border border-[#555]';
            rowLabel.textContent = rank;
            row.appendChild(rowLabel);
            
            // Cells 0-9
            for (let file = 0; file < 10; file++) {
                const cell = document.createElement('div');
                const index = rank * 10 + file; // Índice 0-119 del array
                
                // Colores: borde rojo, casilla vacía beige/marrón, pieza verde
                let bgClass = 'bg-[#b58863]'; // Default: dark square
                if ((rank + file) % 2 === 0) {
                    bgClass = 'bg-[#f0d9b5]'; // Light square
                }
                
                // Bordes (ranks 0,1,10,11 o files 0,9)
                if (rank <= 1 || rank >= 10 || file === 0 || file === 9) {
                    bgClass = 'bg-[#f44336] text-white'; // Red for OFF_BOARD
                }
                
                cell.className = `w-8 h-6 flex items-center justify-center border border-[#555] ${bgClass}`;
                cell.id = `debug-cell-${rank}-${file}`;
                cell.title = `Index: ${index} [${rank}][${file}]`;
                row.appendChild(cell);
            }
            
            grid.appendChild(row);
        }
        
        container.appendChild(grid);
        
        // Info sobre la estructura
        const info = document.createElement('div');
        info.className = 'mt-3 text-xs text-[#888] space-y-1';
        info.innerHTML = `
            <div><strong>Estructura 10x12 (120 elementos):</strong></div>
            <div>• Filas 0-1 y 10-11: <span class="text-[#f44336]">BORDES (0xFF)</span></div>
            <div>• Filas 2-9, Columnas 1-8: <span class="text-[#f0d9b5]">CASILLAS DEL TABLERO</span></div>
            <div>• Columnas 0 y 9: <span class="text-[#f44336]">BORDES (0xFF)</span></div>
            <div>• Ejemplo: Index 21 = [2][1] = a8 (esquina superior izquierda del tablero real)</div>
            <div>• Ejemplo: Index 95 = [9][5] = e1 (posición inicial del rey blanco)</div>
            <div>• Ejemplo: Index 25 = [2][5] = e8 (posición inicial del rey negro)</div>
        `;
        container.appendChild(info);
    }
    
    updateBoardRep(state) {
        if (!state || !state.squares) return;
        
        // Limpiar piezas de todas las casillas (manteniendo colores de fondo)
        for (let rank = 0; rank < 12; rank++) {
            for (let file = 0; file < 10; file++) {
                const cell = document.getElementById(`debug-cell-${rank}-${file}`);
                if (cell) {
                    // Restaurar color base según posición
                    let bgClass = 'bg-[#b58863]'; // Default: dark square
                    if ((rank + file) % 2 === 0) {
                        bgClass = 'bg-[#f0d9b5]'; // Light square
                    }
                    // Bordes
                    if (rank <= 1 || rank >= 10 || file === 0 || file === 9) {
                        bgClass = 'bg-[#f44336] text-white'; // Red for OFF_BOARD
                    }
                    
                    cell.className = `w-8 h-6 flex items-center justify-center border border-[#555] ${bgClass}`;
                    cell.textContent = '';
                }
            }
        }
        
        // Mapeo de piezas a símbolos
        const pieceMap = {
            'king': 'K',
            'queen': 'Q', 
            'rook': 'R',
            'bishop': 'B',
            'knight': 'N',
            'pawn': 'P'
        };
        
        // Colocar piezas en el tablero 10x12
        // El array squares (64 elementos) viene en orden 0-63 (a8-h8, a7-h7, ..., a1-h1)
        // Mapear a posiciones 10x12: fila = floor(index/8) + 2, columna = (index%8) + 1
        state.squares.forEach((piece, index8) => {
            if (piece) {
                const rank10 = Math.floor(index8 / 8) + 2; // +2 porque el tablero empieza en fila 2
                const file10 = (index8 % 8) + 1; // +1 porque el tablero empieza en columna 1
                
                const cell = document.getElementById(`debug-cell-${rank10}-${file10}`);
                if (cell) {
                    let symbol = pieceMap[piece.piece] || '?';
                    if (piece.color === 'black') {
                        symbol = symbol.toLowerCase();
                        cell.classList.add('text-black');
                    } else {
                        cell.classList.add('text-black');
                    }
                    cell.textContent = symbol;
                }
            }
        });
        
        // Mostrar información del último movimiento en el tablero 10x12
        if (state.last_move && state.last_move.from && state.last_move.to) {
            // Convertir notación algebraica (e2) a índices 10x12
            const fromFile = state.last_move.from.charCodeAt(0) - 'a'.charCodeAt(0); // 0-7
            const fromRank = parseInt(state.last_move.from[1]); // 1-8
            const toFile = state.last_move.to.charCodeAt(0) - 'a'.charCodeAt(0); // 0-7
            const toRank = parseInt(state.last_move.to[1]); // 1-8
            
            // Mapear a 10x12: rank va de 8(arriba) a 1(abajo), pero en array es 2-9
            const fromRank10 = (8 - fromRank) + 2;
            const fromFile10 = fromFile + 1;
            const toRank10 = (8 - toRank) + 2;
            const toFile10 = toFile + 1;
            
            const fromCell = document.getElementById(`debug-cell-${fromRank10}-${fromFile10}`);
            const toCell = document.getElementById(`debug-cell-${toRank10}-${toFile10}`);
            
            if (fromCell) {
                fromCell.style.outline = '2px solid #e6c200';
                fromCell.style.outlineOffset = '-2px';
            }
            if (toCell) {
                toCell.style.outline = '2px solid #8aac46';
                toCell.style.outlineOffset = '-2px';
            }
        }
    }
    
    updateMovesDetail(data) {
        const container = document.getElementById('moves-detail');
        container.innerHTML = '';
        
        if (!data || !data.moves) {
            container.innerHTML = '<div style="color: #888;">No hay movimientos disponibles</div>';
            return;
        }
        
        data.moves.forEach(move => {
            const moveEl = document.createElement('div');
            moveEl.className = 'bg-[#2a2a2a] p-2 rounded text-xs font-mono cursor-pointer hover:bg-[#3d3d3d]';
            
            let flags = [];
            if (move.flags === 1) flags.push('enroque K');
            else if (move.flags === 2) flags.push('en passant');
            else if (move.flags === 3) flags.push('doble peón');
            else if (move.flags === 4) flags.push('promoción');
            else if (move.flags === 5) flags.push('enroque Q');
            
            moveEl.innerHTML = `
                <div class="text-white font-bold">${move.san}</div>
                <div class="text-[#888] text-[10px] mt-1">
                    ${move.from}→${move.to} | ${move.piece}${flags.length ? ' | ' + flags.join(', ') : ''}
                </div>
            `;
            
            container.appendChild(moveEl);
        });
        
        // Add count summary
        const summary = document.createElement('div');
        summary.className = 'col-span-full p-2 bg-[#3d3d3d] rounded text-center text-xs';
        summary.innerHTML = `<strong>Total: ${data.count} movimientos legales</strong>`;
        container.appendChild(summary);
    }
    
    updatePerftResults(data) {
        const container = document.getElementById('perft-results');
        
        if (data.error) {
            container.innerHTML = `<pre style="color: #f44336;">Error: ${data.error}</pre>`;
            return;
        }
        
        const nps = data.time_ms > 0 ? Math.round(data.nodes / (data.time_ms / 1000)) : 0;
        
        container.innerHTML = `
<pre>
Perft Depth ${data.depth}
═══════════════════════════
Nodos:      ${data.nodes.toLocaleString()}
Tiempo:     ${data.time_ms} ms
NPS:        ${nps.toLocaleString()}
</pre>`;
    }
    
    logMessage(data, type) {
        const entry = {
            timestamp: new Date().toLocaleTimeString(),
            type: type,
            data: data
        };
        
        this.messageLog.push(entry);
        
        if (this.messageLog.length > this.maxLogEntries) {
            this.messageLog.shift();
        }
        
        this.renderLog();
    }
    
    renderLog() {
        this.logContainer.innerHTML = this.messageLog.map(entry => {
            let typeColor = entry.type === 'sent' ? 'text-[#81b64c]' : entry.type === 'received' ? 'text-[#5dade2]' : 'text-[#f44336]';
            let typeLabel = entry.type === 'sent' ? '→' : entry.type === 'received' ? '←' : '!';
            
            let content = '';
            if (entry.data.type) {
                content = `[${entry.data.type}]`;
            }
            if (entry.data.payload) {
                const payloadStr = JSON.stringify(entry.data.payload).substring(0, 100);
                content += ' ' + payloadStr + (payloadStr.length >= 100 ? '...' : '');
            }
            
            return `<div class="py-0.5 border-b border-[#2a2a2a] ${typeColor}">
                <span class="text-[#666] mr-2">${entry.timestamp}</span>
                <span class="font-bold mr-2">${typeLabel}</span>
                ${content}
            </div>`;
        }).join('');
        
        // Auto-scroll to bottom
        this.logContainer.scrollTop = this.logContainer.scrollHeight;
    }
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new DebugPanel();
});
