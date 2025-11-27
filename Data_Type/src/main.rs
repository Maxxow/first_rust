fn main() {
    let x = 5; // Inmutable
    println!("El valor de x es: {}", x);

    // Si intentas hacer esto, el compilador dará un error:
    // x = 6; // Error: cannot assign twice to immutable variable `x`
    // ¡Inténtalo! Es bueno ver los famosos errores amigables de Rust.
}