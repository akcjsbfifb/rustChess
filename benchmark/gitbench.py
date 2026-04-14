#!/usr/bin/env python3
"""
Git Benchmark - Juega contra cualquier commit de git

Uso:
  python3 gitbench.py --vs-commit abc1234 --games 100
  python3 gitbench.py --vs-commit HEAD~5 --games 50
  python3 gitbench.py --vs-tag v0.1.0 --games 200
"""

import subprocess
import sys
import os
import tempfile
import shutil
from pathlib import Path

# Importar elo module con manejo de error
try:
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    import elo as elo_module
except ImportError:
    elo_module = None

def run_command(cmd, cwd=None, capture=True):
    """Ejecuta comando shell y retorna resultado"""
    try:
        if capture:
            result = subprocess.run(
                cmd, 
                shell=True, 
                cwd=cwd,
                capture_output=True, 
                text=True,
                check=True
            )
            return result.stdout.strip()
        else:
            subprocess.run(cmd, shell=True, cwd=cwd, check=True)
            return None
    except subprocess.CalledProcessError as e:
        print(f"❌ Error ejecutando: {cmd}")
        print(f"   {e}")
        if capture and e.stderr:
            print(f"   stderr: {e.stderr}")
        return None

def get_current_branch_or_commit():
    """Obtiene el commit actual"""
    result = run_command("git rev-parse --short HEAD")
    return result if result else "unknown"

def is_working_directory_clean():
    """Verifica si hay cambios sin commitear"""
    result = run_command("git status --porcelain")
    return result == ""

def stash_changes():
    """Guarda cambios actuales"""
    print("💾 Guardando cambios actuales...")
    run_command("git stash push -m 'gitbench autostash'")

def pop_stash():
    """Restaura cambios guardados"""
    print("📦 Restaurando cambios...")
    run_command("git stash pop")

def checkout_commit(commit_hash):
    """Checkout a un commit específico"""
    print(f"📦 Checkout a commit {commit_hash}...")
    result = run_command(f"git checkout {commit_hash}")
    return result is not None

def return_to_original(original_ref):
    """Vuelve a la referencia original"""
    print(f"📦 Volviendo a {original_ref}...")
    run_command(f"git checkout {original_ref}")

def compile_engine(temp_dir):
    """Compila el motor en un directorio temporal"""
    print("🔨 Compilando motor...")
    
    # Compilar en release
    result = run_command("cargo build --release 2>&1", capture=True)
    
    if result is None:
        return None
    
    # Verificar que se creó el binario
    source = "target/release/rust_chess"
    if not os.path.exists(source):
        print(f"❌ No se encontró binario en {source}")
        return None
    
    # Copiar a directorio temporal
    dest = os.path.join(temp_dir, f"rust_chess_{get_current_branch_or_commit()}")
    shutil.copy(source, dest)
    print(f"✅ Motor compilado: {dest}")
    return dest

def get_commit_info(commit_hash):
    """Obtiene información del commit"""
    msg = run_command(f"git log -1 --pretty=format:'%h %s' {commit_hash}")
    date = run_command(f"git log -1 --pretty=format:'%ar' {commit_hash}")
    return msg, date

def play_match(engine1, engine2, openings_file, games_count, time_per_move=1000):
    """
    Juega un match usando match.py
    
    Retorna: (wins, losses, draws)
    """
    # Importar match.py como módulo
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    try:
        import match as match_module
        
        print(f"\n🏁 Iniciando match: {games_count} partidas")
        print(f"   Motor A: {engine1}")
        print(f"   Motor B: {engine2}")
        
        # Calcular aperturas
        games_per_opening = 2
        num_openings = games_count // games_per_opening
        
        wins, losses, draws = 0, 0, 0
        
        # Leer aperturas
        with open(openings_file, 'r') as f:
            openings = [line.strip() for line in f if line.strip() and not line.startswith('#')]
        
        # Limitar aperturas disponibles
        if num_openings > len(openings):
            print(f"⚠️  Solo {len(openings)} aperturas disponibles, repitiendo algunas...")
            # Repetir aperturas si es necesario
            openings = (openings * ((num_openings // len(openings)) + 1))[:num_openings]
        else:
            openings = openings[:num_openings]
        
        for i, opening in enumerate(openings):
            fen = opening.split(';')[0].strip()
            
            for game_num in range(games_per_opening):
                game_id = i * games_per_opening + game_num + 1
                print(f"  Partida {game_id}/{games_count}...", end=' ', flush=True)
                
                # Alternar colores
                if game_num % 2 == 0:
                    e1, e2 = engine1, engine2
                    is_a_white = True
                else:
                    e1, e2 = engine2, engine1
                    is_a_white = False
                
                # Jugar partida
                try:
                    result, moves, reason = match_module.play_game(e1, e2, fen, time_per_move)
                    
                    # Interpretar resultado
                    if is_a_white:
                        # A juega blancas
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
                        # B juega blancas (resultado invertido)
                        if result == 1:
                            losses += 1  # B ganó
                            print(f"B gana ({len(moves)} movs)")
                        elif result == 0:
                            wins += 1  # B perdió
                            print(f"A gana ({len(moves)} movs)")
                        else:
                            draws += 1
                            print(f"Empate ({len(moves)} movs)")
                
                except Exception as e:
                    print(f"❌ Error en partida: {e}")
                    draws += 1  # Contar como empate en caso de error
        
        return wins, losses, draws
        
    except ImportError:
        print("❌ No se pudo importar match.py")
        return None, None, None

def calculate_elo(wins, losses, draws):
    """Calcula Elo usando elo.py"""
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    try:
        import elo as elo_module
        result = elo_module.calculate_elo(wins, losses, draws)
        return result
    except ImportError:
        return None

def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description='Benchmark contra cualquier commit de git',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Ejemplos:
  # Jugar contra commit específico
  python3 gitbench.py --vs-commit a1b2c3d
  
  # Jugar contra 5 commits atrás
  python3 gitbench.py --vs-commit HEAD~5
  
  # Jugar contra tag
  python3 gitbench.py --vs-tag v0.1.0
  
  # Comparar dos commits específicos (no actual vs viejo)
  python3 gitbench.py --engine1-commit abc1234 --engine2-commit def5678
  
  # Más partidas para mejor precisión
  python3 gitbench.py --vs-commit HEAD~10 --games 200
        """
    )
    
    # Opciones de qué comparar
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--vs-commit', help='Commit hash contra el que jugar (vs versión actual)')
    group.add_argument('--vs-tag', help='Tag contra el que jugar')
    group.add_argument('--engine1-commit', help='Commit para motor A')
    group.add_argument('--engine2-commit', help='Commit para motor B (requiere --engine1-commit)')
    
    # Opciones adicionales
    parser.add_argument('--games', type=int, default=100, help='Número de partidas (default: 100)')
    parser.add_argument('--openings', default='openings.epd', help='Archivo de aperturas')
    parser.add_argument('--time', type=int, default=1000, help='Tiempo por movimiento en ms (default: 1000)')
    parser.add_argument('--keep-builds', action='store_true', help='No borrar motores compilados')
    
    args = parser.parse_args()
    
    # Verificar que estamos en el repo correcto
    if not os.path.exists('../src/main.rs') and not os.path.exists('src/main.rs'):
        print("❌ Debes ejecutar esto desde el directorio del proyecto rust_chess")
        print(f"   Actual: {os.getcwd()}")
        sys.exit(1)
    
    # Determinar commits a comparar
    if args.vs_commit or args.vs_tag:
        # Motor A = versión actual
        # Motor B = commit especificado
        commit_a = "HEAD"  # Actual
        commit_b = args.vs_commit or args.vs_tag
        label_a = "actual (HEAD)"
        label_b = f"commit {commit_b}"
    else:
        # Comparar dos commits específicos
        if not args.engine1_commit or not args.engine2_commit:
            print("❌ Debes especificar ambos commits: --engine1-commit y --engine2-commit")
            sys.exit(1)
        commit_a = args.engine1_commit
        commit_b = args.engine2_commit
        label_a = f"commit {commit_a}"
        label_b = f"commit {commit_b}"
    
    # Guardar referencia actual
    original_ref = run_command("git rev-parse --abbrev-ref HEAD")
    if not original_ref:
        original_ref = run_command("git rev-parse HEAD")
    
    print("=" * 70)
    print("GIT BENCHMARK - Comparando versiones")
    print("=" * 70)
    print(f"Motor A: {label_a}")
    print(f"Motor B: {label_b}")
    print(f"Partidas: {args.games}")
    print()
    
    # Verificar working directory
    if not is_working_directory_clean():
        print("⚠️  Tienes cambios sin commitear")
        response = input("¿Querés guardarlos (stash) y continuar? [Y/n]: ").strip().lower()
        if response in ['', 'y', 'yes']:
            stash_changes()
            stashed = True
        else:
            print("❌ Cancelado. Commiteá o stashéa tus cambios primero.")
            sys.exit(1)
    else:
        stashed = False
    
    # Crear directorio temporal
    temp_dir = tempfile.mkdtemp(prefix='rust_chess_bench_')
    print(f"📁 Directorio temporal: {temp_dir}")
    
    engine_a = None
    engine_b = None
    
    try:
        # Compilar Motor A
        print(f"\n🔨 Compilando Motor A ({label_a})...")
        if commit_a != "HEAD":
            if not checkout_commit(commit_a):
                raise Exception(f"No se pudo checkout a {commit_a}")
        
        engine_a = compile_engine(temp_dir)
        if not engine_a:
            raise Exception("Falló compilación del Motor A")
        
        # Volver a original para compilar Motor B
        if commit_a != "HEAD":
            return_to_original(original_ref)
        
        # Compilar Motor B
        print(f"\n🔨 Compilando Motor B ({label_b})...")
        if not checkout_commit(commit_b):
            raise Exception(f"No se pudo checkout a {commit_b}")
        
        engine_b = compile_engine(temp_dir)
        if not engine_b:
            raise Exception("Falló compilación del Motor B")
        
        # Volver a original
        return_to_original(original_ref)
        
        # Jugar match
        print("\n" + "=" * 70)
        wins, losses, draws = play_match(engine_a, engine_b, args.openings, args.games, args.time)
        
        if wins is None:
            raise Exception("Error jugando partidas")
        
        # Mostrar resultados
        print("\n" + "=" * 70)
        print("RESULTADOS DEL MATCH")
        print("=" * 70)
        print(f"Motor A ({label_a}): {wins} victorias")
        print(f"Empates: {draws}")
        print(f"Motor B ({label_b}): {losses} victorias")
        print(f"Total: {wins + draws + losses} partidas")
        print()
        
        # Calcular Elo
        result = calculate_elo(wins, losses, draws)
        if result and elo_module:
            print(elo_module.format_result(result))
        else:
            print(f"\nCalculá Elo manualmente:")
            print(f"python3 elo.py --wins {wins} --losses {losses} --draws {draws}")
        
        # Guardar binarios si se pidió
        if args.keep_builds:
            final_a = f"./rust_chess_{Path(engine_a).stem}"
            final_b = f"./rust_chess_{Path(engine_b).stem}"
            shutil.copy(engine_a, final_a)
            shutil.copy(engine_b, final_b)
            print(f"\n💾 Motores guardados:")
            print(f"   {final_a}")
            print(f"   {final_b}")
        
    except Exception as e:
        print(f"\n❌ Error: {e}")
        # Asegurar que volvemos al estado original
        return_to_original(original_ref)
        if stashed:
            pop_stash()
        sys.exit(1)
    
    finally:
        # Limpieza
        if not args.keep_builds:
            print(f"\n🧹 Limpiando {temp_dir}...")
            shutil.rmtree(temp_dir, ignore_errors=True)
        
        # Restaurar stash si es necesario
        if stashed:
            pop_stash()
        
        # Asegurar que estamos en el ref original
        current = run_command("git rev-parse --abbrev-ref HEAD")
        if current != original_ref:
            return_to_original(original_ref)
    
    print("\n✅ Benchmark completado!")

if __name__ == "__main__":
    main()
