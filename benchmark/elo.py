#!/usr/bin/env python3
"""
Calculadora de Elo para chess engines
Usa el modelo Bradley-Terry para estimar Elo difference
"""

import sys
import argparse
import math

def calculate_elo(wins: int, losses: int, draws: int) -> dict:
    """
    Calcula Elo difference y error margins
    
    Fórmula: Elo_diff = -400 * log10((1 - score) / score)
    donde score = (wins + 0.5 * draws) / total
    
    Error margin (95% confidence): 1.96 * sigma / sqrt(n)
    """
    total = wins + losses + draws
    
    if total == 0:
        return None
    
    # Score (0 a 1)
    score = (wins + 0.5 * draws) / total
    
    # Elo difference (si score > 0.5, somos mejores)
    if score <= 0 or score >= 1:
        elo_diff = float('inf') if score >= 1 else float('-inf')
    else:
        elo_diff = -400 * math.log10((1 - score) / score)
    
    # Error margin (95% confidence interval)
    if score <= 0 or score >= 1:
        error_margin = 0
    else:
        # Usando fórmula aproximada para Elo
        # sigma^2 = p * (1-p) / n
        variance = score * (1 - score) / total
        sigma = math.sqrt(variance)
        # Multiplicamos por 400 y por 1.96 para 95% CI
        error_margin = 400 * 1.96 * sigma * 2  # *2 porque es diferencia
    
    # Win rates
    win_rate = wins / total * 100
    draw_rate = draws / total * 100
    loss_rate = losses / total * 100
    
    # Likelihood of superiority (LOS)
    # Probabilidad de que motor A sea mejor que motor B
    if wins + losses == 0:
        los = 50.0  # Solo empates
    else:
        # Aproximación normal
        n = wins + losses
        p = wins / n
        sigma_p = math.sqrt(p * (1 - p) / n)
        if sigma_p == 0:
            los = 100.0 if wins > losses else 0.0 if losses > wins else 50.0
        else:
            z = (p - 0.5) / sigma_p
            # Aproximación de la CDF normal
            los = (1 + math.erf(z / math.sqrt(2))) / 2 * 100
    
    return {
        'wins': wins,
        'losses': losses,
        'draws': draws,
        'total': total,
        'score': score,
        'win_rate': win_rate,
        'draw_rate': draw_rate,
        'loss_rate': loss_rate,
        'elo_diff': elo_diff,
        'error_margin': error_margin,
        'los': los,
        'ci_lower': elo_diff - error_margin,
        'ci_upper': elo_diff + error_margin
    }

def format_result(result: dict) -> str:
    """Formatea resultado para mostrar"""
    if result is None:
        return "Error: No hay partidas"
    
    lines = []
    lines.append("=" * 60)
    lines.append("RESULTADOS DEL BENCHMARK")
    lines.append("=" * 60)
    lines.append("")
    lines.append(f"Partidas: {result['total']}")
    lines.append(f"  + Victorias: {result['wins']} ({result['win_rate']:.1f}%)")
    lines.append(f"  = Empates:    {result['draws']} ({result['draw_rate']:.1f}%)")
    lines.append(f"  - Derrotas:   {result['losses']} ({result['loss_rate']:.1f}%)")
    lines.append("")
    lines.append(f"Score: {result['score']:.3f} ({result['wins'] + 0.5*result['draws']:.1f}/{result['total']})")
    lines.append("")
    lines.append("ELO ESTIMADO")
    lines.append("-" * 60)
    
    if result['elo_diff'] == float('inf'):
        lines.append("Elo: +∞ (invicto!)")
    elif result['elo_diff'] == float('-inf'):
        lines.append("Elo: -∞ (sin victorias)")
    else:
        lines.append(f"Diferencia Elo: {result['elo_diff']:+.1f} ± {result['error_margin']:.1f}")
        lines.append(f"Intervalo 95%: [{result['ci_lower']:+.1f}, {result['ci_upper']:+.1f}]")
    
    lines.append("")
    lines.append(f"LOS (Likelihood of Superiority): {result['los']:.1f}%")
    lines.append("  (Probabilidad de que el motor A sea mejor que B)")
    lines.append("")
    
    if result['los'] > 95:
        lines.append("✅ CONCLUSIÓN: El motor A es SIGNIFICATIVAMENTE MEJOR")
    elif result['los'] < 5:
        lines.append("❌ CONCLUSIÓN: El motor A es SIGNIFICATIVAMENTE PEOR")
    else:
        lines.append("➡️  CONCLUSIÓN: No hay diferencia significativa (necesitás más partidas)")
    
    lines.append("=" * 60)
    
    return "\n".join(lines)

def main():
    parser = argparse.ArgumentParser(description='Calculadora de Elo para engines')
    parser.add_argument('--wins', type=int, required=True, help='Victorias motor A')
    parser.add_argument('--losses', type=int, required=True, help='Derrotas motor A')
    parser.add_argument('--draws', type=int, required=True, help='Empates')
    
    args = parser.parse_args()
    
    result = calculate_elo(args.wins, args.losses, args.draws)
    print(format_result(result))
    
    # Return code for CI/automation
    if result and result['los'] > 95:
        return 0  # Success, A is better
    elif result and result['los'] < 5:
        return 1  # Failure, A is worse
    return 2  # Inconclusive

if __name__ == "__main__":
    sys.exit(main())
