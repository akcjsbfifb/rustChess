# Cómo Funciona GitBench Internamente

## Diagrama del Flujo

```
TU DIRECTORIO DE TRABAJO (/home/marto/Work/rust_chess/)
│
├── src/              ← Tu código actual (con cambios o sin)
├── target/release/   ← Binario actual (HEAD)
└── .git/             ← Repo git
    │
    └── (estado actual: rama main, commit 68159ec)


EJECUTÁS: python3 benchmark/gitbench.py --vs-commit HEAD~5


1. GUARDAR CAMBIOS ACTUALES (si los hay)
   │
   └── git stash push -m 'gitbench autostash'
       
       Resultado:
       - Tus cambios van al stash
       - Working directory queda limpio
       - Tu código NO se pierde


2. COMPILAR MOTOR A (versión actual - HEAD)
   │
   └── cargo build --release
       │
       └── target/release/rust_chess (binario actual)
       │
       └── COPIAR a: /tmp/rust_chess_bench_XXXXX/rust_chess_68159ec

   IMPORTANTE: No hace checkout, usa tu código actual


3. CHECKOUT AL COMMIT VIEJO
   │
   └── git checkout e09e59a  (HEAD~5)
       │
       ├── src/ cambia a la versión vieja
       ├── Cargo.toml de esa época
       └── Todo el código antiguo
       
       TU RAMA ACTUAL: "(HEAD detached at e09e59a)"


4. COMPILAR MOTOR B (versión vieja - HEAD~5)
   │
   └── cargo build --release
       │
       └── target/release/rust_chess (binario viejo)
       │
       └── COPIAR a: /tmp/rust_chess_bench_XXXXX/rust_chess_e09e59a


5. VOLVER A TU RAMA
   │
   └── git checkout main
       │
       └── src/ vuelve a tu código actual
       └── Todo está como antes


6. RESTAURAR TUS CAMBIOS (si los había)
   │
   └── git stash pop
       │
       └── Tu código vuelve al working directory


7. JUGAR LAS PARTIDAS
   │
   └── Usa los dos binarios en /tmp/:
       ├── Motor A: /tmp/rust_chess_bench_XXXXX/rust_chess_68159ec
       └── Motor B: /tmp/rust_chess_bench_XXXXX/rust_chess_e09e59a
       
       NO toca tu target/release/ actual


8. LIMPIEZA
   │
   └── rm -rf /tmp/rust_chess_bench_XXXXX/
       (a menos que uses --keep-builds)
```

---

## Respuestas a tus preguntas

### 1. ¿El checkout afecta mi git actual?

**NO**, tu repo queda exactamente igual que antes:

```bash
# Antes del benchmark:
git status
# On branch main
# Your branch is ahead of 'origin/main' by 6 commits

# Durante el benchmark:
git checkout HEAD~5  # ← Esto cambia working directory
# (Detached HEAD)

# Después del benchmark:
git checkout main    # ← Vuelve a main
git stash pop        # ← Restaura tus cambios (si los había)

# Tu repo queda:
git status
# On branch main
# Your branch is ahead of 'origin/main' by 6 commits
# (exactamente igual que antes)
```

**Tu código, tus commits, tu historial = intactos.**

---

### 2. ¿Dónde queda el ejecutable viejo?

En un **directorio temporal** en `/tmp/`:

```bash
# Durante el benchmark:
/tmp/rust_chess_bench_a1b2c3d/
├── rust_chess_68159ec   ← Versión actual (HEAD)
└── rust_chess_e09e59a   ← Versión vieja (HEAD~5)
```

**No está en tu directorio de trabajo**, está en `/tmp/` (sistema).

Después de terminar, se borra automáticamente (a menos que uses `--keep-builds`).

---

### 3. ¿Puedo quedarme con los ejecutables?

**Sí**, usá `--keep-builds`:

```bash
python3 benchmark/gitbench.py --vs-commit HEAD~5 --games 100 --keep-builds
```

Esto hace que al final, en vez de borrar, copie los binarios a tu directorio:

```bash
/home/marto/Work/rust_chess/
├── src/
├── target/
├── benchmark/
├── rust_chess_68159ec    ← Motor actual (guardado)
└── rust_chess_e09e59a    ← Motor viejo (guardado)
```

Y podés usarlos después:

```bash
# Jugar otra vez sin recompilar:
cd benchmark
python3 match.py \
  --engine1 ../rust_chess_68159ec \
  --engine2 ../rust_chess_e09e59a \
  --games 50
```

---

### 4. ¿Qué pasa si el benchmark se interrumpe?

Si apretás Ctrl+C o hay un error:

```python
try:
    # ... benchmark ...
except:
    # Siempre ejecuta esto:
    git checkout main      # Vuelve a main
    git stash pop          # Restaura tus cambios
    cleanup_temp_files()   # Limpia /tmp/
```

El `try/finally` asegura que **siempre** vuelvas a tu estado original.

---

### 5. ¿Puedo seguir trabajando mientras corre?

**NO**, porque el benchmark:
1. Hace `git checkout` (cambia tu working directory)
2. Usa `cargo build` (modifica `target/`)
3. Si vos editás archivos durante eso, se rompe todo

**Solución:** Dejá que termine, o abrí otra terminal para trabajar en paralelo en otro directorio.

---

## Ejemplo paso a paso visual

```bash
$ cd /home/marto/Work/rust_chess

# Veo mi estado actual
$ git status
On branch main
Your branch is ahead of 'origin/main' by 6 commits
nothing to commit, working tree clean

# Ejecuto benchmark
$ python3 benchmark/gitbench.py --vs-commit HEAD~5 --games 10

=== DURANTE EL BENCHMARK ===

1. [GitBench] Guardando cambios...
   → git stash (no hay nada que guardar, working tree clean)

2. [GitBench] Compilando Motor A (HEAD)... 
   → cargo build --release
   → Copiando a /tmp/rust_chess_bench_XXX/rust_chess_68159ec
   
3. [GitBench] Checkout a HEAD~5...
   → git checkout e09e59a
   
   # Tu working directory ahora tiene código viejo
   # Pero esto es TEMPORAL

4. [GitBench] Compilando Motor B (HEAD~5)...
   → cargo build --release
   → Copiando a /tmp/rust_chess_bench_XXX/rust_chess_e09e59a

5. [GitBench] Volviendo a main...
   → git checkout main
   
   # Tu working directory vuelve a tener tu código actual

6. [GitBench] Restaurando cambios...
   → git stash pop (no hay stash, nada que restaurar)

7. [GitBench] Jugando partidas...
   → Usa: /tmp/rust_chess_bench_XXX/rust_chess_68159ec
   → Usa: /tmp/rust_chess_bench_XXX/rust_chess_e09e59a
   → Tu src/ y target/ NO se tocan

8. [GitBench] Limpieza...
   → rm -rf /tmp/rust_chess_bench_XXX/

=== DESPUÉS DEL BENCHMARK ===

$ git status
On branch main
Your branch is ahead of 'origin/main' by 6 commits
nothing to commit, working tree clean

$ ls -la rust_chess_* 2>/dev/null
# No hay nada (salvo que uses --keep-builds)

$ ls -la /tmp/ | grep rust_chess
# No hay nada (ya se limpió)

# Tu repo está EXACTAMENTE igual que antes
```

---

## Resumen

| Pregunta | Respuesta |
|----------|-----------|
| ¿Afecta mi git? | **NO**, siempre vuelve a main y restaura stash |
| ¿Dónde queda el binario viejo? | `/tmp/rust_chess_bench_XXX/` (temporal) |
| ¿Puedo guardar los binarios? | **Sí**, con `--keep-builds` |
| ¿Puedo trabajar durante el benchmark? | **NO**, mejor esperar o usar otra terminal |
| ¿Es seguro? | **SÍ**, usa try/finally para garantizar cleanup |

---

## Comando seguro para probar

```bash
# 10 partidas rápido, no afecta nada:
cd /home/marto/Work/rust_chess
python3 benchmark/gitbench.py --vs-commit HEAD~1 --games 10

# Verificá después:
git status  # Debe estar igual que antes
```

**El benchmark es 100% seguro y no toca tu trabajo.**
