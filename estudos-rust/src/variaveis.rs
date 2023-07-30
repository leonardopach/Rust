pub fn variaveis() {
    // Variaveis são imutaveis por padrão
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);

    const PONTOS_MAXIMO: u32 = 100_000;
    println!("{PONTOS_MAXIMO}");

    // Shadowing
    let y = 9;

    println!("O valor de y é: {y}");
    let y = 10;

    println!("O valor de y depois do Shadowing é: {y}");

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("O valor de x é: {x}");

    let espacao = "    ";
    let espacao = espacao.len();
    println!("Espaço: {espacao}");
}
