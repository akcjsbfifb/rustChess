class ChessBoard {
    constructor() {
        this.board = document.getElementById('board');
        this.selectedSquare = null;
        this.validMoves = [];
        this.isFlipped = false;
        this.currentFEN = '';
        this.legalMoves = [];
        
        this.pieceSymbols = {
            'king': 'k',
            'queen': 'q',
            'rook': 'r',
            'bishop': 'b',
            'knight': 'n',
            'pawn': 'p'
        };
        
        this.pieceImages = {};
        this.boardImage = '/images/boards/green.png';
        
        this.init();
    }
    
    init() {
        this.createBoard();
        this.setupEventListeners();
        this.setupWebSocketHandlers();
        
        // Start WebSocket connection
        chessWS.connect();
    }
    
    createBoard() {
        this.board.innerHTML = '';
        
        const files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        const ranks = ['8', '7', '6', '5', '4', '3', '2', '1'];
        
        // Crear contenedor flex para el tablero
        const boardWrapper = document.createElement('div');
        boardWrapper.className = 'flex flex-col items-center';
        
        // Fila de coordenadas superiores
        const topCoords = document.createElement('div');
        topCoords.className = 'flex h-5 w-[500px]';
        const filesToShow = this.isFlipped ? files.slice().reverse() : files;
        topCoords.innerHTML = '<div class="w-5 flex-shrink-0"></div>' + 
            filesToShow.map(f => `<div class="w-[60px] flex items-center justify-center text-xs text-[#888] font-bold flex-shrink-0">${f}</div>`).join('') +
            '<div class="w-5 flex-shrink-0"></div>';
        boardWrapper.appendChild(topCoords);
        
        // Área principal del tablero
        const mainArea = document.createElement('div');
        mainArea.className = 'flex w-[500px]';
        
        // Coordenadas izquierda
        const leftCoords = document.createElement('div');
        leftCoords.className = 'flex flex-col w-5 flex-shrink-0';
        for (let i = 0; i < 8; i++) {
            const rankIdx = this.isFlipped ? (7 - i) : i;
            const rankNum = document.createElement('div');
            rankNum.className = 'h-[60px] flex items-center justify-center text-xs text-[#888] font-bold flex-shrink-0';
            rankNum.textContent = ranks[rankIdx];
            leftCoords.appendChild(rankNum);
        }
        mainArea.appendChild(leftCoords);
        
        // El tablero grid
        const boardGrid = document.createElement('div');
        boardGrid.className = 'board-grid border-2 border-[#404040] rounded';
        
        for (let visualRank = 0; visualRank < 8; visualRank++) {
            for (let visualFile = 0; visualFile < 8; visualFile++) {
                const square = document.createElement('div');
                square.className = 'square';
                
                const isLight = (visualRank + visualFile) % 2 === 0;
                square.classList.add(isLight ? 'light' : 'dark');
                
                const logicalRank = this.isFlipped ? (7 - visualRank) : visualRank;
                const logicalFile = this.isFlipped ? (7 - visualFile) : visualFile;
                
                const logicalIndex = logicalRank * 8 + logicalFile;
                const squareName = files[logicalFile] + ranks[logicalRank];
                
                square.dataset.square = squareName;
                square.dataset.index = logicalIndex;
                
                square.addEventListener('click', (e) => this.handleSquareClick(e, square));
                
                boardGrid.appendChild(square);
            }
        }
        mainArea.appendChild(boardGrid);
        
        // Coordenadas derecha
        const rightCoords = document.createElement('div');
        rightCoords.className = 'flex flex-col w-5 flex-shrink-0';
        for (let i = 0; i < 8; i++) {
            const rankIdx = this.isFlipped ? (7 - i) : i;
            const rankNum = document.createElement('div');
            rankNum.className = 'h-[60px] flex items-center justify-center text-xs text-[#888] font-bold flex-shrink-0';
            rankNum.textContent = ranks[rankIdx];
            rightCoords.appendChild(rankNum);
        }
        mainArea.appendChild(rightCoords);
        
        boardWrapper.appendChild(mainArea);
        
        // Fila de coordenadas inferiores
        const bottomCoords = document.createElement('div');
        bottomCoords.className = 'flex h-5 w-[500px]';
        bottomCoords.innerHTML = '<div class="w-5 flex-shrink-0"></div>' + 
            filesToShow.map(f => `<div class="w-[60px] flex items-center justify-center text-xs text-[#888] font-bold flex-shrink-0">${f}</div>`).join('') +
            '<div class="w-5 flex-shrink-0"></div>';
        boardWrapper.appendChild(bottomCoords);
        
        this.board.appendChild(boardWrapper);
    }
    
    setupEventListeners() {
        document.getElementById('btn-new-game').addEventListener('click', () => {
            chessWS.send({ type: 'init', payload: { fen: '' } });
        });
        
        document.getElementById('btn-undo').addEventListener('click', () => {
            chessWS.send({ type: 'undo' });
        });
        
        document.getElementById('btn-flip').addEventListener('click', () => {
            this.isFlipped = !this.isFlipped;
            this.createBoard();
            this.updateBoard(this.lastState);
        });
        
        document.getElementById('btn-engine-play').addEventListener('click', () => {
            chessWS.send({ type: 'engine_go' });
            document.getElementById('engine-info').innerHTML = '<div>Pensando...</div>';
        });
        
        document.getElementById('btn-copy-fen').addEventListener('click', () => {
            const fenInput = document.getElementById('fen-input');
            fenInput.select();
            document.execCommand('copy');
            
            const btn = document.getElementById('btn-copy-fen');
            const originalText = btn.textContent;
            btn.textContent = 'Copiado!';
            setTimeout(() => btn.textContent = originalText, 1000);
        });
        
        // Cargar FEN personalizado
        document.getElementById('btn-load-fen').addEventListener('click', () => {
            const fenInput = document.getElementById('fen-load-input');
            const fen = fenInput.value.trim();
            
            if (fen) {
                chessWS.send({ type: 'init', payload: { fen: fen } });
                fenInput.value = ''; // Limpiar después de cargar
                this.updateStatus('Cargando posición FEN...');
            } else {
                this.updateStatus('Por favor ingresa un FEN válido');
            }
        });
        
        // Cargar posición inicial
        document.getElementById('btn-startpos').addEventListener('click', () => {
            chessWS.send({ type: 'init', payload: { fen: '' } });
            this.updateStatus('Cargando posición inicial...');
        });
    }
    
    setupWebSocketHandlers() {
        chessWS.on('board_state', (state) => {
            this.updateBoard(state);
            chessWS.send({ type: 'get_moves' });
        });
        
        chessWS.on('legal_moves', (data) => {
            this.legalMoves = data.moves || [];
            this.updateMovesList();
        });
        
        chessWS.on('best_move', (data) => {
            this.updateEngineInfo(data);
        });
        
        chessWS.on('command_response', (data) => {
            if (data.error) {
                this.updateStatus('Error: ' + data.error);
            }
        });
        
        chessWS.on('error', (data) => {
            this.updateStatus('Error: ' + data.message);
        });
    }
    
    updateBoard(state) {
        if (!state) return;
        
        this.lastState = state;
        this.currentFEN = state.fen;
        
        // Update FEN input
        document.getElementById('fen-input').value = state.fen;
        
        // Update turn indicator
        const turnText = state.turn === 'white' ? 'Turno: Blancas' : 'Turno: Negras';
        document.getElementById('turn-indicator').textContent = turnText;
        
        // Update last move
        const lastMoveEl = document.getElementById('last-move');
        if (state.last_move) {
            lastMoveEl.textContent = `Último movimiento: ${state.last_move.from} → ${state.last_move.to}`;
        } else {
            lastMoveEl.textContent = '';
        }
        
        // Clear board
        const grid = this.board.querySelector('.board-grid');
        if (grid) {
            const squares = grid.querySelectorAll('.square');
            squares.forEach(sq => {
                sq.innerHTML = '';
                sq.classList.remove('last-move');
            });
        }
        
        // Place pieces
        if (state.squares) {
            state.squares.forEach((piece, index) => {
                if (piece) {
                    const square = this.getSquareByIndex(index);
                    if (square) {
                        const pieceCode = this.pieceSymbols[piece.piece];
                        const colorCode = piece.color === 'white' ? 'w' : 'b';
                        const imgPath = `/images/pieces/${colorCode}${pieceCode}.png?v=2`;
                        
                        const pieceEl = document.createElement('img');
                        pieceEl.className = `piece ${piece.color}`;
                        pieceEl.src = imgPath;
                        pieceEl.alt = `${piece.color} ${piece.piece}`;
                        pieceEl.draggable = false;
                        pieceEl.onerror = function() {
                            console.error('Failed to load piece image:', imgPath);
                        };
                        square.appendChild(pieceEl);
                    }
                }
            });
        }
        
        // Highlight last move
        if (state.last_move) {
            const fromIdx = this.squareToIndex(state.last_move.from);
            const toIdx = this.squareToIndex(state.last_move.to);
            
            const fromSq = this.getSquareByIndex(fromIdx);
            const toSq = this.getSquareByIndex(toIdx);
            
            if (fromSq) fromSq.classList.add('last-move');
            if (toSq) toSq.classList.add('last-move');
        }
        
        this.updateStatus('Listo');
    }
    
    getSquareByIndex(index) {
        const grid = this.board.querySelector('.board-grid');
        return grid ? grid.querySelector(`[data-index="${index}"]`) : null;
    }
    
    getSquareByName(name) {
        const grid = this.board.querySelector('.board-grid');
        return grid ? grid.querySelector(`[data-square="${name}"]`) : null;
    }
    
    squareToIndex(square) {
        if (!square || square.length !== 2) return -1;
        
        const file = square.charCodeAt(0) - 'a'.charCodeAt(0);
        const rank = 8 - parseInt(square[1]);
        
        // Devuelve índice lógico (0-63), independiente de la orientación visual
        return rank * 8 + file;
    }
    
    indexToSquare(index) {
        const file = String.fromCharCode('a'.charCodeAt(0) + (index % 8));
        const rank = 8 - Math.floor(index / 8);
        return file + rank;
    }
    
    handleSquareClick(event, square) {
        const squareName = square.dataset.square;
        const piece = square.querySelector('.piece');
        console.log('Clicked square:', squareName, 'Has piece:', !!piece);
        
        // If we have a selected square and this is a valid move
        if (this.selectedSquare) {
            console.log('Have selected square, checking for valid move to:', squareName);
            const move = this.validMoves.find(m => m.to === squareName);
            console.log('Found move:', move);
            
            if (move) {
                // Make the move
                this.makeMove(this.selectedSquare.dataset.square, squareName);
                this.clearSelection();
                return;
            }
        }
        
        // If clicking on own piece, select it
        if (piece) {
            const isWhitePiece = piece.classList.contains('white');
            const isWhiteTurn = this.lastState && this.lastState.turn === 'white';
            console.log('Clicked piece - isWhite:', isWhitePiece, 'isWhiteTurn:', isWhiteTurn);
            
            if (isWhitePiece === isWhiteTurn) {
                this.selectSquare(square);
            } else {
                this.clearSelection();
            }
        } else {
            this.clearSelection();
        }
    }
    
    selectSquare(square) {
        this.clearSelection();
        
        this.selectedSquare = square;
        square.classList.add('selected');
        
        // Find valid moves from this square
        const fromSquare = square.dataset.square;
        console.log('Selecting square:', fromSquare);
        console.log('All legal moves:', this.legalMoves);
        this.validMoves = this.legalMoves.filter(m => m.from === fromSquare);
        console.log('Valid moves from this square:', this.validMoves);
        
        // Highlight valid moves
        this.validMoves.forEach(move => {
            const targetSquare = this.getSquareByName(move.to);
            console.log('Highlighting square:', move.to, targetSquare);
            if (targetSquare) {
                targetSquare.classList.add('valid-move');
                if (targetSquare.querySelector('.piece')) {
                    targetSquare.classList.add('has-piece');
                }
            }
        });
    }
    
    clearSelection() {
        if (this.selectedSquare) {
            this.selectedSquare.classList.remove('selected');
        }
        
        const grid = this.board.querySelector('.board-grid');
        if (grid) {
            grid.querySelectorAll('.valid-move').forEach(sq => {
                sq.classList.remove('valid-move', 'has-piece');
            });
        }
        
        this.selectedSquare = null;
        this.validMoves = [];
    }
    
    makeMove(from, to) {
        const moveStr = from + to;
        chessWS.send({ type: 'make_move', payload: { move: moveStr } });
        this.updateStatus('Moviendo...');
    }
    
    updateMovesList() {
        const movesList = document.getElementById('moves-list');
        movesList.innerHTML = '';
        
        this.legalMoves.forEach(move => {
            const moveEl = document.createElement('div');
            moveEl.className = 'move-item';
            moveEl.textContent = `${move.san} (${move.from}-${move.to})`;
            moveEl.addEventListener('click', () => {
                this.makeMove(move.from, move.to);
            });
            movesList.appendChild(moveEl);
        });
    }
    
    updateEngineInfo(data) {
        const infoEl = document.getElementById('engine-info');
        
        if (data.error) {
            infoEl.innerHTML = `<div style="color: #f44336;">${data.error}</div>`;
            return;
        }
        
        const depth = data.depth || 0;
        const nodes = data.nodes || 0;
        const time = data.time_ms || 0;
        const evalScore = data.eval || 0;
        const bestMove = data.best_move || 'N/A';
        
        const nps = time > 0 ? Math.round(nodes / (time / 1000)) : 0;
        const evalText = evalScore > 0 ? `+${evalScore/100}` : `${evalScore/100}`;
        
        infoEl.innerHTML = `
            <div><strong>Mejor movimiento:</strong> ${bestMove}</div>
            <div><strong>Evaluación:</strong> ${evalText}</div>
            <div><strong>Profundidad:</strong> ${depth}</div>
            <div><strong>Nodos:</strong> ${nodes.toLocaleString()}</div>
            <div><strong>NPS:</strong> ${nps.toLocaleString()}</div>
            <div><strong>Tiempo:</strong> ${time}ms</div>
        `;
    }
    
    updateStatus(message) {
        const statusBox = document.getElementById('status-box');
        statusBox.innerHTML = `<div>${message}</div>`;
    }
}

// Initialize board when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new ChessBoard();
});
