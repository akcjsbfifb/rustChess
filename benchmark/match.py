#!/usr/bin/env python3
"""
Match runner - Hace que dos versiones del motor jueguen entre sí
"""

import subprocess
import json
import sys
import random
from pathlib import Path

def run_engine(engine_path):
    """Inicia el motor y retorna procesos stdin/stdout"""
    process = subprocess.Popen(
        [engine_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )
    return process

def is_process_alive(process):
    """Verifica si el proceso sigue corriendo"""
    if process is None:
        return False
    return process.poll() is None

def send_command(process, cmd):
    """Envía comando al motor y retorna respuesta JSON"""
    if not is_process_alive(process):
        return {"error": "Process is dead"}
    
    try:
        process.stdin.write(cmd + "\n")
        process.stdin.flush()
    except (BrokenPipeError, OSError, ValueError) as e:
        return {"error": f"Failed to send command: {e}"}
    
    if not is_process_alive(process):
        return {"error": "Process died after sending command"}
    
    try:
        response = process.stdout.readline().strip()
    except (BrokenPipeError, OSError, ValueError) as e:
        return {"error": f"Failed to read response: {e}"}
    
    try:
        return json.loads(response)
    except json.JSONDecodeError:
        return {"raw": response}

def is_pawn_move(move):
    """Detecta si un movimiento es de peón (e.g., e2e4, a7a8q)"""
    if len(move) < 4:
        return False
    from_square = move[0:2]
    return from_square[1] in '27'  # Peones en filas 2 y 7

def is_capture(move):
    """Detecta captura por 'x' en notación estándar (no aplica a nuestro formato)"""
    # En nuestro formato simple (e2e4), detectamos capturas por cambio de archivo
    # o simplemente asumimos que el engine detectará captura de rey
    return 'x' in move

def play_game(engine1_path, engine2_path, opening_fen, time_per_move=1000, max_moves=200):
    """
    Juega una partida entre dos motores
    
    Args:
        engine1_path: Path a motor A (juega blancas primero)
        engine2_path: Path a motor B (juega negras)
        opening_fen: Posición inicial (FEN)
        time_per_move: Tiempo por movimiento en ms
        max_moves: Límite de seguridad (default 200)
    
    Returns:
        (result, moves, reason)
        result: 1 (A gana), 0.5 (empate), 0 (B gana)
        moves: Lista de movimientos
        reason: Por qué terminó la partida
    """
    engine1 = None
    engine2 = None
    
    try:
        # Iniciar motores
        engine1 = run_engine(engine1_path)
        engine2 = run_engine(engine2_path)
        
        # Inicializar
        send_command(engine1, "uci")
        send_command(engine2, "uci")
        
        # Setear posición inicial
        send_command(engine1, f"position fen {opening_fen}")
        send_command(engine2, f"position fen {opening_fen}")
        
        moves = []
        side_to_move = 1
        move_count = 0
        halfmove_clock = 0  # Contador para regla de 50 movimientos
        position_history = []  # Para detectar repetición
        
        while move_count < max_moves:
            current_engine = engine1 if side_to_move == 1 else engine2
            
            # Pedir movimiento al motor actual
            response = send_command(current_engine, f"go depth 4")
            
            # Verificar error
            if "error" in response:
                return 0.5, moves, f"engine_error_side_{side_to_move}"
            
            if "best_move" not in response:
                # No hay movimientos legales
                if side_to_move == 1:
                    return 0, moves, "engine1_no_moves"
                else:
                    return 1, moves, "engine2_no_moves"
            
            best_move = response["best_move"]
            if not best_move or best_move == "":
                return 0.5, moves, "empty_move"
            
            moves.append(best_move)
            
            # Actualizar contador de 50 movimientos
            if is_pawn_move(best_move) or len(best_move) > 4:  # Promoción implica peón
                halfmove_clock = 0
            else:
                halfmove_clock += 1
            
            # Verificar regla de 50 movimientos
            if halfmove_clock >= 100:  # 100 medios movimientos = 50 jugadas completas
                return 0.5, moves, "fifty_move_rule"
            
            # Enviar movimiento a ambos motores
            other_engine = engine2 if side_to_move == 1 else engine1
            res1 = send_command(other_engine, f"move {best_move}")
            res2 = send_command(current_engine, f"move {best_move}")
            
            if "error" in res1 or "error" in res2:
                return 0.5, moves, "sync_error"
            
            side_to_move = 3 - side_to_move
            move_count += 1
        
        # Límite de seguridad alcanzado
        return 0.5, moves, "max_moves_reached"
        
    finally:
        # Asegurar limpieza de procesos
        if engine1:
            try:
                engine1.terminate()
                engine1.wait(timeout=2)
            except:
                try:
                    engine1.kill()
                except:
                    pass
        if engine2:
            try:
                engine2.terminate()
                engine2.wait(timeout=2)
            except:
                try:
                    engine2.kill()
                except:
                    pass

def play_match(engine1, engine2, openings_file, games_per_opening=2):
    """
    Juega un match completo con múltiples aperturas
    
    Args:
        engine1: Path a motor A
        engine2: Path a motor B
        openings_file: Archivo con posiciones EPD
        games_per_opening: Cuántas veces jugar cada apertura (normalmente 2 para alternar colores)
    """
    
    # Leer aperturas
    with open(openings_file, 'r') as f:
        openings = [line.strip() for line in f if line.strip() and not line.startswith('#')]
    
    print(f"Jugando {len(openings)} aperturas × {games_per_opening} partidas = {len(openings) * games_per_opening} total")
    print(f"Motor A: {engine1}")
    print(f"Motor B: {engine2}")
    print()
    
    wins = 0
    losses = 0
    draws = 0
    
    for i, opening in enumerate(openings):
        # Extraer FEN (primera parte de la línea EPD)
        fen = opening.split(';')[0].strip()
        
        for game_num in range(games_per_opening):
            print(f"Apertura {i+1}/{len(openings)}, partida {game_num+1}/{games_per_opening}... ", end='', flush=True)
            
            try:
                # Alternar colores
                if game_num % 2 == 0:
                    e1, e2 = engine1, engine2
                    result, moves, reason = play_game(e1, e2, fen)
                    
                    if result == 1:
                        wins += 1
                        print(f"A gana ({len(moves)} movs)")
                    elif result == 0:
                        losses += 1
                        print(f"B gana ({len(moves)} movs)")
                    else:
                        draws += 1
                        print(f"Empate ({len(moves)} movs) [{reason}]")
                else:
                    # Segunda partida: B juega blancas
                    e1, e2 = engine2, engine1
                    result, moves, reason = play_game(e1, e2, fen)
                    
                    # Invertir resultado porque intercambiamos colores
                    if result == 1:
                        losses += 1  # B ganó, es derrota para A
                        print(f"B gana ({len(moves)} movs)")
                    elif result == 0:
                        wins += 1  # B perdió, es victoria para A
                        print(f"A gana ({len(moves)} movs)")
                    else:
                        draws += 1
                        print(f"Empate ({len(moves)} movs) [{reason}]")
            except Exception as e:
                draws += 1
                print(f"ERROR ({e})")
                continue
    
    print()
    print("=" * 60)
    print("RESULTADOS DEL MATCH")
    print("=" * 60)
    print(f"Motor A: {wins} victorias")
    print(f"Empates: {draws}")
    print(f"Motor B: {losses} victorias")
    print(f"Total: {wins + draws + losses} partidas")
    print()
    print("Ahora podés calcular Elo con:")
    print(f"python3 elo.py --wins {wins} --losses {losses} --draws {draws}")
    print()

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description='Hace que dos motores jueguen entre sí')
    parser.add_argument('--engine1', required=True, help='Path al motor A (versión nueva)')
    parser.add_argument('--engine2', required=True, help='Path al motor B (versión vieja/baseline)')
    parser.add_argument('--openings', default='openings.epd', help='Archivo de aperturas')
    parser.add_argument('--games', type=int, default=100, help='Número total de partidas')
    
    args = parser.parse_args()
    
    # Calcular cuántas aperturas necesitamos
    games_per_opening = 2  # Para alternar colores
    num_openings = args.games // games_per_opening
    
    # Limitar al número de aperturas disponibles
    with open(args.openings, 'r') as f:
        available_openings = len([l for l in f if l.strip() and not l.startswith('#')])
    
    if num_openings > available_openings:
        print(f"Advertencia: Pediste {num_openings} aperturas pero solo hay {available_openings}")
        print(f"Repitiendo algunas aperturas...")
        num_openings = available_openings
    
    play_match(args.engine1, args.engine2, args.openings, games_per_opening)
