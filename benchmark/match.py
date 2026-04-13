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

def send_command(process, cmd):
    """Envía comando al motor y retorna respuesta JSON"""
    process.stdin.write(cmd + "\n")
    process.stdin.flush()
    response = process.stdout.readline().strip()
    try:
        return json.loads(response)
    except:
        return {"raw": response}

def play_game(engine1_path, engine2_path, opening_fen, time_per_move=1000):
    """
    Juega una partida entre dos motores
    
    Args:
        engine1_path: Path a motor A (juega blancas primero)
        engine2_path: Path a motor B (juega negras)
        opening_fen: Posición inicial (FEN)
        time_per_move: Tiempo por movimiento en ms
    
    Returns:
        (result, moves, reason)
        result: 1 (A gana), 0.5 (empate), 0 (B gana)
        moves: Lista de movimientos
        reason: Por qué terminó la partida
    """
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
    side_to_move = 1  # 1 = engine1 (white), 2 = engine2 (black)
    move_count = 0
    max_moves = 200  # Límite de seguridad
    
    while move_count < max_moves:
        current_engine = engine1 if side_to_move == 1 else engine2
        
        # Pedir movimiento al motor actual
        response = send_command(current_engine, f"go depth 4")
        
        if "best_move" not in response:
            # No hay movimientos legales - juego terminado
            # Verificar si es jaque mate o ahogado
            state = send_command(current_engine, "state")
            
            # Determinar resultado
            if side_to_move == 1:
                # Engine1 no tiene movimientos - pierde
                return 0, moves, "engine1_no_moves"
            else:
                # Engine2 no tiene movimientos - pierde  
                return 1, moves, "engine2_no_moves"
        
        best_move = response["best_move"]
        moves.append(best_move)
        
        # Enviar movimiento al otro motor
        other_engine = engine2 if side_to_move == 1 else engine1
        send_command(other_engine, f"move {best_move}")
        
        # También actualizar motor actual
        send_command(current_engine, f"move {best_move}")
        
        side_to_move = 3 - side_to_move  # Alternar 1 <-> 2
        move_count += 1
        
        # Verificar si alguien capturó el rey (simplificación)
        # En ajedrez real no se captura el rey, pero para detectar mate:
        if move_count > 10:  # Dar tiempo para desarrollo
            state = send_command(engine1 if side_to_move == 1 else engine2, "state")
            # Aquí iría lógica de detección de mate/repetición/50 movimientos
    
    # Si llegamos a max_moves, es empate por límite
    return 0.5, moves, "max_moves_reached"

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
                    print(f"Empate ({len(moves)} movs)")
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
                    print(f"Empate ({len(moves)} movs)")
    
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
