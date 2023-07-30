fn main() {
    let numero = 3;

    if numero < 5 {
        println!("condição era verdadeiro");
    } else {
        println!("Condição era falsa");
    }

    let condicao = true;
    let numero = if condicao { 5 } else { 6 };
    println!("O valor do número é: {}", numero);

    let mut numero = 3;

    while numero != 0 {
        println!("{}", numero);

        numero = numero - 1;
    }

    let a = [10, 20, 30, 40, 50];

    for elemento in a.iter() {
        println!("{elemento}");
    }

    for numero in (1..5).rev() {
        println!("{}", numero);
    }
}
