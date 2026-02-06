# Progreso de Aprendizaje - Rust Book

## Última actualización: 2026-02-05

## Completado ✅

### Capítulo 0 - Introducción
- ✅ Prólogo (Foreword) - Bec Rumbul
- ✅ Capítulo 0 - Introducción

### Capítulo 1 - Getting Started
- ✅ ch01-00-getting-started.md - Introducción al capítulo
- ✅ ch01-01-installation.md - Instalación (Rust 1.92.0 ya instalado)
  - rustup, rustc, cargo
  - Troubleshooting y verificación
- ✅ ch01-02-hello-world.md - Primer programa
  - Estructura básica de un programa Rust
  - Compilación con rustc
  - Macros (println!)
- ✅ ch01-03-hello-cargo.md - Sistema de build
  - cargo new, build, run, check
  - Estructura de proyecto Cargo
  - Debug vs Release builds

### Capítulo 2 - Programming a Guessing Game
**ESTADO: COMPLETADO** - Código final funcionando en `guessing_game/`
- ✅ ch02-00-guessing-game-tutorial.md
  - Setting Up a New Project
  - Processing a Guess (Listing 2-1)
    - use std::io
    - Variables mutables (let mut)
    - String::new() - associated functions
    - io::stdin().read_line()
    - Referencias (&mut)
    - Result y expect
    - Placeholders en println!
  - Generating a Secret Number (Listing 2-2, 2-3)
    - Agregando dependencias (rand = "0.8.5")
    - Cargo.lock y builds reproducibles
    - Semantic Versioning
    - Traits (Rng)
    - Método gen_range()
  - Comparing the Guess to the Secret Number (Listing 2-4)
    - std::cmp::Ordering (enum: Less, Greater, Equal)
    - match expressions
    - Type mismatch errors
    - Shadowing de variables
    - trim(), parse()
    - Anotaciones de tipo (u32)
    - Inferencia de tipos
  - Allowing Multiple Guesses with Looping (Listing 2-5, 2-6)
    - loop (bucles infinitos)
    - break (salir del loop)
    - continue (saltar iteración)
    - Manejo de errores con match
    - Patrón catch-all (_)
  - Código final completo implementado y probado

### Capítulo 3 - Common Programming Concepts
**ESTADO: COMPLETADO**
- ✅ ch03-00-common-programming-concepts.md
- ✅ ch03-01-variables-and-mutability.md
- ✅ ch03-02-data-types.md
- ✅ ch03-03-how-functions-work.md
- ✅ ch03-04-comments.md
- ✅ ch03-05-control-flow.md

## En Curso 📖

### Capítulo 4 - Understanding Ownership
**PRÓXIMO A INICIAR**
- 📖 ch04-00-understanding-ownership.md
- 📖 ch04-01-what-is-ownership.md
- 📖 ch04-02-references-and-borrowing.md
- 📖 ch04-03-slices.md

## Pendiente ⏳

### Capítulo 5 - Structs
- ⏳ ch05-00-structs.md
- ⏳ ch05-01-defining-structs.md
- ⏳ ch05-02-example-structs.md
- ⏳ ch05-03-method-syntax.md

### Capítulo 6 - Enums and Pattern Matching
- ⏳ ch06-00-enums.md
- ⏳ ch06-01-defining-an-enum.md
- ⏳ ch06-02-match.md
- ⏳ ch06-03-if-let.md

### Capítulo 7 - Managing Growing Projects
- ⏳ ch07-00-managing-growing-projects-with-packages-crates-and-modules.md
- ⏳ ch07-01-packages-and-crates.md
- ⏳ ch07-02-defining-modules-to-control-scope-and-privacy.md
- ⏳ ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md
- ⏳ ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
- ⏳ ch07-05-separating-modules-into-different-files.md

### Capítulos 8-21
- ⏳ Capítulo 8 - Common Collections
- ⏳ Capítulo 9 - Error Handling
- ⏳ Capítulo 10 - Generics, Traits, and Lifetimes
- ⏳ Capítulo 11 - Testing
- ⏳ Capítulo 12 - An I/O Project (grep)
- ⏳ Capítulo 13 - Functional Language Features
- ⏳ Capítulo 14 - More About Cargo
- ⏳ Capítulo 15 - Smart Pointers
- ⏳ Capítulo 16 - Fearless Concurrency
- ⏳ Capítulo 17 - Async/Await
- ⏳ Capítulo 18 - Object-Oriented Programming
- ⏳ Capítulo 19 - Patterns and Matching
- ⏳ Capítulo 20 - Advanced Features
- ⏳ Capítulo 21 - Final Project (Web Server)

## Notas del Progreso

### Proyectos Completados
- `guessing_game/` - Juego de adivinanza funcional con:
  - Número aleatorio entre 1-100
  - Múltiples intentos
  - Comparaciones (muy alto/muy bajo)
  - Manejo de errores (input no numérico)
  - Loop hasta acertar

### Conceptos Clave Aprendidos
1. **Cargo**: Sistema de build y gestor de paquetes
2. **Variables**: Inmutables por defecto, mutables con `mut`
3. **Tipos**: String, u32, i32, Result, Ordering
4. **Control de flujo**: match, loop, break, continue
5. **Referencias**: &mut para préstamo mutable
6. **Shadowing**: Reutilizar nombres de variables
7. **Traits**: Rng para números aleatorios
8. **Error handling**: Result<T, E> con match
9. **Crates externos**: rand y gestión de dependencias

### Configuración del Entorno
- Rust version: 1.92.0 (Arch Linux)
- Editor: Cualquiera (libro no asume IDE específico)
- Documentación local: `rustup doc` o `cargo doc --open`

### Próximos Pasos
Continuar con Capítulo 3: Common Programming Concepts
- Variables y mutabilidad
- Tipos de datos
- Funciones
- Comentarios
- Control de flujo
