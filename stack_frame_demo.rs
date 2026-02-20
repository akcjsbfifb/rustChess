fn main() {
    // Todas estas variables se reservan EN UN SOLO PASO
    // al entrar a main(), no una por una
    let a = 10i32; // 4 bytes
    let b = 20i32; // 4 bytes
    let c = 30i32; // 4 bytes
    let d = 40i32; // 4 bytes
    let e = 50i32; // 4 bytes

    println!("a={}, b={}, c={}, d={}, e={}", a, b, c, d, e);

    // El compilador calcula: necesito 20 bytes para todas las variables
    // Reserva TODO de una vez al entrar a main()

    ejemplo();
}

fn ejemplo() {
    let x = 100i64; // 8 bytes
    let y = 200i64; // 8 bytes
    let z = 300i64; // 8 bytes

    // Stack frame de ejemplo(): 24 bytes totales
    // Se reservan todos juntos, no uno por uno

    println!("x={}, y={}, z={}", x, y, z);
}
