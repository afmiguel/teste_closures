fn main() {
    // Definindo um closure que aceita um argumento `x` e retorna `x + 1`
    let incrementa = |x: i32| x + 1;

    // Chamando o closure
    let resultado = incrementa(5);
    println!("O resultado é: {}", resultado);  // Saída: O resultado é: 6

    // Definindo um closure que captura uma variável do ambiente
    let y = 10;
    let adiciona_y = |x: i32| x + y;

    // Chamando o closure
    let resultado = adiciona_y(5);
    println!("O resultado é: {}", resultado);  // Saída: O resultado é: 15

    // Passando um closure como argumento para uma função
    let resultado = aplica_func(5, incrementa);
    println!("O resultado é: {}", resultado);  // Saída: O resultado é: 6
}

// Definindo uma função que aceita um i32 e um closure como argumentos
fn aplica_func(x: i32, f: impl Fn(i32) -> i32) -> i32 {
    f(x)  // Chama o closure `f` com o argumento `x`
}
