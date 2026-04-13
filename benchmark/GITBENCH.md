# Git Benchmark - Jugar contra cualquier commit

## ¿Qué hace esto?

`gitbench.py` te permite jugar contra **cualquier versión** de tu motor almacenada en git:

- Un commit específico (`a1b2c3d`)
- Un tag (`v0.1.0`)
- 5 commits atrás (`HEAD~5`)
- Comparar dos commits entre sí

## Instalación

El script está en `benchmark/gitbench.py` y no requiere instalación.

```bash
cd /home/marto/Work/rust_chess/benchmark
python3 gitbench.py --help
```

## Uso básico

### 1. Jugar contra un commit específico

```bash
python3 gitbench.py --vs-commit 857b963 --games 50
```

Esto:
1. Guarda tus cambios actuales (si hay)
2. Compila la versión actual (HEAD) → Motor A
3. Checkout al commit `857b963`
4. Compila esa versión → Motor B
5. Juega 50 partidas entre ellos
6. Calcula Elo
7. Vuelve a tu rama original
8. Restaura tus cambios

### 2. Jugar contra versión anterior

```bash
# Contra 3 commits atrás
python3 gitbench.py --vs-commit HEAD~3 --games 100

# Contra el commit anterior
python3 gitbench.py --vs-commit HEAD~1 --games 50
```

### 3. Jugar contra un tag

```bash
python3 gitbench.py --vs-tag v0.1.0 --games 100
```

### 4. Comparar dos commits específicos

```bash
# Compara commit abc1234 vs def5678
python3 gitbench.py \
  --engine1-commit abc1234 \
  --engine2-commit def5678 \
  --games 100
```

## Ejemplos prácticos

### Escenario 1: "¿Mis cambios mejoran el motor?"

Estás trabajando en una nueva feature (ej: mejor PST). Querés saber si realmente mejora.

```bash
# Primero, commiteá tus cambios actuales
cd /home/marto/Work/rust_chess
git add .
git commit -m "feat: mejores PST tables"

# Ahora compará contra la versión anterior
python3 benchmark/gitbench.py --vs-commit HEAD~1 --games 100

# Output esperado:
# Motor A (actual HEAD): 55 victorias
# Empates: 20
# Motor B (HEAD~1): 35 victorias
# 
# ELO ESTIMADO: +73.2 ± 64.8
# ✅ Motor A es significativamente mejor
```

### Escenario 2: "¿Cuánto mejoré desde la versión inicial?"

```bash
# Ver commits disponibles
git log --oneline -10

# Jugar contra el primer commit funcional
python3 benchmark/gitbench.py --vs-commit 1e11b3a --games 200

# Output:
# Motor A (actual): 142 victorias
# Empates: 18
# Motor B (inicial): 40 victorias
#
# ELO: +287.4 ± 42.1
# ✅ Ganaste ~287 Elo desde la versión inicial!
```

### Escenario 3: "¿Este commit específico era bueno?"

```bash
# Revisar historial
git log --oneline --graph

# Probar commit específico
python3 benchmark/gitbench.py --vs-commit a930823 --games 50
```

## Opciones avanzadas

### Más partidas para mejor precisión

```bash
python3 gitbench.py --vs-commit HEAD~1 --games 500
```

### Guardar los motores compilados

```bash
python3 gitbench.py --vs-commit HEAD~5 --games 100 --keep-builds
# Crea:
# ./rust_chess_abc1234 (versión A)
# ./rust_chess_def5678 (versión B)
```

### Aperturas personalizadas

```bash
python3 gitbench.py \
  --vs-commit HEAD~1 \
  --games 100 \
  --openings mis_aperturas.epd
```

## Flujo de trabajo recomendado

### 1. Desarrollo iterativo

```bash
# Hacés un cambio
# ... editás código ...

# Lo commiteás
git add src/
git commit -m "feat: mejor quiescence"

# Lo testeás contra la versión anterior
python3 benchmark/gitbench.py --vs-commit HEAD~1 --games 100

# Si el SPRT dice "accept_h1", tu cambio es bueno
# Si dice "accept_h0", revertí el cambio
# Si dice "continue", necesitás más partidas
```

### 2. Regresión testing

```bash
# Antes de un release importante, testeá contra múltiples versiones

# Test 1: Contra versión anterior
python3 benchmark/gitbench.py --vs-commit HEAD~1 --games 200

# Test 2: Contra versión hace 1 semana
python3 benchmark/gitbench.py --vs-commit HEAD~20 --games 100

# Test 3: Contra versión inicial
python3 benchmark/gitbench.py --vs-tag v0.1.0 --games 50
```

### 3. Bisect para encontrar cuándo se rompió algo

```bash
# Si encontrás que el motor empeoró, usá git bisect

# Primero, encontrar un commit bueno y uno malo
git log --oneline

# Supongamos:
# abc1234 = último commit bueno (+200 Elo)
# def5678 = primer commit malo (-50 Elo)

# Hacés bisect manual con gitbench
git checkout abc1234
python3 benchmark/gitbench.py --vs-tag v0.1.0 --games 50  # Baseline

# Guardás resultado
git checkout def5678
python3 benchmark/gitbench.py --vs-tag v0.1.0 --games 50

# Comparás resultados para ver cuál es mejor
```

## Troubleshooting

### "Tengo cambios sin commitear"

El script te avisa y pregunta si querés hacer stash:

```
⚠️  Tienes cambios sin commitear
¿Querés guardarlos (stash) y continuar? [Y/n]: 
```

Respondé `Y` y el script:
1. Guarda tus cambios (`git stash`)
2. Hace el benchmark
3. Restaura tus cambios (`git stash pop`)

### "El commit no existe"

```bash
# Verificá que el commit existe
git log --oneline | grep a930823

# Si no está, quizás hiciste force push
# Buscá en el reflog
git reflog | head -20
```

### "La compilación falló"

Esto puede pasar si:
1. El commit tiene código roto
2. Faltan dependencias en Cargo.toml
3. El commit es de antes de que existiera algún archivo

**Solución:** Probá con un commit más reciente, o fixeá el código manualmente.

### "Volví a main pero no tengo mis cambios"

Si algo sale mal durante el benchmark, podés perder el stash.

**Recuperación:**
```bash
# Ver stashes disponibles
git stash list

# Recuperar
git stash pop stash@{0}

# Si no está, buscar en reflog
git reflog show stash
```

## Casos de uso avanzados

### Integración con CI/CD

```yaml
# .github/workflows/benchmark.yml
jobs:
  benchmark:
    steps:
      - uses: actions/checkout@v2
      
      - name: Benchmark vs previous commit
        run: |
          cd benchmark
          python3 gitbench.py --vs-commit HEAD~1 --games 100 > results.txt
          
          # Extraer resultado
          if grep -q "ACEPTAR H1" results.txt; then
            echo "✅ Cambio aprobado"
          elif grep -q "ACEPTAR H0" results.txt; then
            echo "❌ Cambio rechazado - regresión detectada"
            exit 1
          else
            echo "⚠️  Inconcluso - necesita más testing"
          fi
```

### Testing de múltiples features

```bash
#!/bin/bash
# test_all_features.sh

FEATURES=("HEAD~1" "HEAD~2" "HEAD~5" "a930823")
BASELINE="1e11b3a"

for feature in "${FEATURES[@]}"; do
  echo "Testing feature: $feature"
  python3 benchmark/gitbench.py \
    --engine1-commit "$feature" \
    --engine2-commit "$BASELINE" \
    --games 50 \
    --keep-builds
done

# Comparar todos los resultados
```

### Torneo entre commits

```bash
#!/bin/bash
# tournament.sh

# 4 mejores commits recientes
COMMITS=("HEAD" "HEAD~1" "HEAD~2" "HEAD~3")

for ((i=0; i<${#COMMITS[@]}; i++)); do
  for ((j=i+1; j<${#COMMITS[@]}; j++)); do
    echo "Match: ${COMMITS[$i]} vs ${COMMITS[$j]}"
    python3 benchmark/gitbench.py \
      --engine1-commit "${COMMITS[$i]}" \
      --engine2-commit "${COMMITS[$j]}" \
      --games 50
  done
done
```

## Referencia rápida

| Comando | Qué hace |
|---------|----------|
| `--vs-commit abc1234` | Juega HEAD vs ese commit |
| `--vs-commit HEAD~5` | Juega vs 5 commits atrás |
| `--vs-tag v0.1.0` | Juega vs tag específico |
| `--engine1-commit A --engine2-commit B` | Compara dos commits |
| `--games 200` | Juega 200 partidas (default 100) |
| `--keep-builds` | Guarda los binarios compilados |
| `--time 2000` | 2 segundos por movimiento |

---

## Tips finales

1. **Commiteá frecuentemente**: Más commits = más puntos de referencia para comparar
2. **Usá tags para releases**: `git tag v0.2.0` y después compará fácil
3. **Guardá los builds importantes**: `--keep-builds` para versiones históricas
4. **Documentá los resultados**: Anotá Elo ganado en el mensaje de commit

---

## Ejemplo completo de sesión

```bash
$ cd /home/marto/Work/rust_chess

# Hago un cambio...
$ vim src/search.rs

# Lo testeo localmente
$ cargo test
# ... tests pasan ...

# Lo commiteo
$ git add src/search.rs
$ git commit -m "feat: mejor delta pruning en quiescence"

# Ahora lo comparo contra la versión anterior
$ python3 benchmark/gitbench.py --vs-commit HEAD~1 --games 100

======================================================================
GIT BENCHMARK - Comparando versiones
======================================================================
Motor A: actual (HEAD)
Motor B: commit 857b963
Partidas: 100

💾 Guardando cambios actuales...
📁 Directorio temporal: /tmp/rust_chess_bench_abc123/

🔨 Compilando Motor A (actual HEAD)...
✅ Motor compilado: /tmp/rust_chess_bench_abc123/rust_chess_9f8e7d6

📦 Volviendo a main

🔨 Compilando Motor B (commit 857b963)...
✅ Motor compilado: /tmp/rust_chess_bench_abc123/rust_chess_857b963

======================================================================
🏁 Iniciando match: 100 partidas

  Partida 1/100... A gana (45 movs)
  Partida 2/100... Empate (67 movs)
  Partida 3/100... A gana (38 movs)
  ...
  Partida 100/100... A gana (52 movs)

======================================================================
RESULTADOS DEL MATCH
======================================================================
Motor A (actual HEAD): 58 victorias
Empates: 22
Motor B (commit 857b963): 30 victorias
Total: 110 partidas

============================================================
RESULTADOS DEL BENCHMARK
============================================================
Partidas: 110
  + Victorias: 58 (52.7%)
  = Empates:    22 (20.0%)
  - Derrotas:   30 (27.3%)

ELO ESTIMADO
------------------------------------------------------------
Diferencia Elo: +87.3 ± 62.1
Intervalo 95%: [25.2, 149.4]

LOS (Likelihood of Superiority): 97.8%

✅ CONCLUSIÓN: El motor A es SIGNIFICATIVAMENTE MEJOR
============================================================

✅ Benchmark completado!

# Excelente! Mi cambio de delta pruning ganó ~87 Elo
# Está confirmado estadísticamente (LOS 97.8% > 95%)
```

---

**Ahora podés comparar cualquier versión de tu motor con confianza estadística! 🎉**
