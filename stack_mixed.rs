fn main() {
    // Diferentes tamaños en el mismo stack frame
    let byte_val: u8 = 1; // 1 byte
    let int_val: i32 = 100; // 4 bytes
    let big_val: i64 = 1000; // 8 bytes
    let flag: bool = true; // 1 byte

    // El compilador calcula el espacio necesario (con padding por alineación)
    println!(
        "byte={}, int={}, big={}, flag={}",
        byte_val, int_val, big_val, flag
    );
}
