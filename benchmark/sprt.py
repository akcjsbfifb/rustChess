#!/usr/bin/env python3
"""
SPRT (Sequential Probability Ratio Test) para chess engines

El SPRT permite decidir cuándo parar las pruebas basándose en estadística.
En lugar de jugar un número fijo de partidas, el SPRT te dice:
- "Aceptar H1: El cambio mejora el motor"
- "Aceptar H0: El cambio no mejora (o empeora)"
- "Continuar testeando: No hay suficiente evidencia aún"

Parámetros:
- elo0: Elo mínimo aceptable (ej: 0, no queremos regresión)
- elo1: Elo que queremos detectar (ej: 5 o 10, mejora significativa)
- alpha: Probabilidad de falso positivo (default 0.05)
- beta: Probabilidad de falso negativo (default 0.05)
"""

import sys
import argparse
import math

def llr(wins, losses, draws, elo0, elo1):
    """
    Log-Likelihood Ratio para SPRT
    
    Fórmula: LLR = sum(log(p1(xi) / p0(xi)))
    
    Donde:
    - p0: probabilidad de victoria bajo elo0
    - p1: probabilidad de victoria bajo elo1
    """
    if wins + losses + draws == 0:
        return 0.0
    
    # Convertir Elo a probabilidades
    # P(victoria) = 1 / (1 + 10^(-elo/400))
    p0 = 1.0 / (1.0 + math.pow(10.0, -elo0 / 400.0))
    p1 = 1.0 / (1.0 + math.pow(10.0, -elo1 / 400.0))
    
    # Probabilidades observadas
    total = wins + losses + draws
    w = wins / total
    l = losses / total
    d = draws / total
    
    # Log-likelihood ratio
    # Si p1 > p0, estamos buscando evidencia de mejora
    if w > 0:
        llr_w = w * math.log((2 * p1) / (2 * p0))
    else:
        llr_w = 0
    
    if l > 0:
        llr_l = l * math.log((2 * (1 - p1)) / (2 * (1 - p0)))
    else:
        llr_l = 0
    
    if d > 0:
        # Para empates, modelo simplificado
        llr_d = d * math.log((1 - p1 * p1) / (1 - p0 * p0))
    else:
        llr_d = 0
    
    return (llr_w + llr_l + llr_d) * total

def sprt_bounds(alpha=0.05, beta=0.05):
    """
    Límites de decisión para SPRT
    
    A = log(beta / (1 - alpha))
    B = log((1 - beta) / alpha)
    
    Si LLR < A: Aceptar H0 (no hay mejora)
    Si LLR > B: Aceptar H1 (hay mejora)
    Si A < LLR < B: Continuar testeando
    """
    lower = math.log(beta / (1 - alpha))
    upper = math.log((1 - beta) / alpha)
    return lower, upper

def sprt(wins, losses, draws, elo0=0, elo1=10, alpha=0.05, beta=0.05):
    """
    Realiza el test SPRT
    
    Returns: (decision, llr_value, bounds)
    - decision: 'accept_h0', 'accept_h1', 'continue'
    - llr_value: valor actual del LLR
    - bounds: (lower, upper) límites
    """
    result = llr(wins, losses, draws, elo0, elo1)
    lower, upper = sprt_bounds(alpha, beta)
    
    if result < lower:
        return 'accept_h0', result, (lower, upper)
    elif result > upper:
        return 'accept_h1', result, (lower, upper)
    else:
        return 'continue', result, (lower, upper)

def elo_variance(wins, losses, draws):
    """Calcula varianza de estimador Elo"""
    total = wins + losses + draws
    if total < 2:
        return float('inf')
    
    score = (wins + 0.5 * draws) / total
    # Varianza aproximada
    return 4 * score * (1 - score) / total

def main():
    parser = argparse.ArgumentParser(
        description='SPRT (Sequential Probability Ratio Test) para engines',
        epilog='Ejemplo: python3 sprt.py --wins 30 --losses 20 --draws 10 --elo0 0 --elo1 10'
    )
    parser.add_argument('--wins', type=int, required=True, help='Victorias')
    parser.add_argument('--losses', type=int, required=True, help='Derrotas')
    parser.add_argument('--draws', type=int, required=True, help='Empates')
    parser.add_argument('--elo0', type=float, default=0, help='Elo mínimo aceptable (default: 0)')
    parser.add_argument('--elo1', type=float, default=10, help='Elo a detectar (default: 10)')
    parser.add_argument('--alpha', type=float, default=0.05, help='Probabilidad falso positivo')
    parser.add_argument('--beta', type=float, default=0.05, help='Probabilidad falso negativo')
    
    args = parser.parse_args()
    
    decision, llr_val, (lower, upper) = sprt(
        args.wins, args.losses, args.draws,
        args.elo0, args.elo1, args.alpha, args.beta
    )
    
    total = args.wins + args.losses + args.draws
    score = (args.wins + 0.5 * args.draws) / total if total > 0 else 0
    
    print("=" * 70)
    print("SPRT - SEQUENTIAL PROBABILITY RATIO TEST")
    print("=" * 70)
    print()
    print(f"Resultados: {args.wins}W/{args.draws}D/{args.losses}L (Total: {total})")
    print(f"Score: {score:.3f}")
    print()
    print("Parámetros del test:")
    print(f"  Elo0 (hipótesis nula):     {args.elo0:+.1f}")
    print(f"  Elo1 (hipótesis alternativa): {args.elo1:+.1f}")
    print(f"  Alpha (falso positivo):    {args.alpha:.3f}")
    print(f"  Beta (falso negativo):     {args.beta:.3f}")
    print()
    print("LLR (Log-Likelihood Ratio):")
    print(f"  Valor actual: {llr_val:.2f}")
    print(f"  Límite inferior (A): {lower:.2f}")
    print(f"  Límite superior (B): {upper:.2f}")
    print()
    
    # Barra de progreso visual
    width = 50
    if decision == 'continue':
        # Escalar llr para mostrar en barra
        range_val = upper - lower
        position = (llr_val - lower) / range_val
        pos = int(position * width)
        pos = max(0, min(width, pos))
        bar = "[" + "=" * pos + ">" + " " * (width - pos - 1) + "]"
        print(f"Progreso: {bar}")
        print(f"          {lower:.1f} {bar} {upper:.1f}")
    print()
    
    if decision == 'accept_h1':
        print("✅ RESULTADO: ACEPTAR H1")
        print()
        print(f"El cambio mejora el motor por al menos +{args.elo1} Elo")
        print(f"Confianza: {(1 - args.alpha)*100:.1f}%")
        print()
        print("🎉 El cambio es SIGNIFICATIVAMENTE BUENO. ¡Aprobar!")
        return 0
    
    elif decision == 'accept_h0':
        print("❌ RESULTADO: ACEPTAR H0")
        print()
        print(f"El cambio NO mejora el motor (o es peor que +{args.elo0} Elo)")
        print(f"Confianza: {(1 - args.beta)*100:.1f}%")
        print()
        print("⚠️  El cambio NO es significativamente mejor. Revisar o descartar.")
        return 1
    
    else:  # continue
        print("⏳ RESULTADO: CONTINUAR TESTEANDO")
        print()
        print("No hay suficiente evidencia estadística aún.")
        print()
        # Estimar partidas necesarias
        if llr_val < 0:
            # Más cerca de A, podría necesitar partidas para confirmar regresión
            est_games = total * (abs(lower) / (abs(llr_val) + 0.01))
            print(f"💡 Estimación: necesitás ~{int(est_games)} partidas más para decisión")
        else:
            est_games = total * (upper / (llr_val + 0.01))
            print(f"💡 Estimación: necesitás ~{int(est_games)} partidas más para decisión")
        print()
        print("📝 Seguir jugando partidas hasta alcanzar un límite")
        return 2

if __name__ == "__main__":
    sys.exit(main())
