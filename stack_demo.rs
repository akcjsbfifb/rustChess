fn main() {
    let x = 5; // Variable en stack de main
    println!("1. main: x = {}", x);

    let result = function_a(x); // Llamamos a function_a
    println!("5. main: result = {}", result);

    let y = 10; // Otra variable
    println!("6. main: y = {}", y);
} // main termina, todo el stack se libera

fn function_a(num: i32) -> i32 {
    println!("2. function_a: num = {}", num);

    let local_a = num * 2; // Variable local en stack
    println!("3. function_a: local_a = {}", local_a);

    let result_b = function_b(local_a); // Llamamos a function_b
    println!("4. function_a: result_b = {}", result_b);

    result_b + 1 // Retornamos valor
} // function_a termina, su stack frame se destruye

fn function_b(value: i32) -> i32 {
    println!("   -> function_b: value = {}", value);

    let local_b = value + 100; // Variable local
    println!("   -> function_b: local_b = {}", local_b);

    local_b // Retornamos
} // function_b termina, su stack frame se destruye
