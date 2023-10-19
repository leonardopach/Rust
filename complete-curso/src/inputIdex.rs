use std::io;

fn inputIndex() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("Por favor, insira um índice para o array: ");

    let mut index = String::new();

    match io::stdin().read_line(&mut index) {
        Ok(_) => {
            let index: usize = match index.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Índice inserido não era um número válido");
                    return;
                }
            };

            if index < a.len() {
                let element = a[index];
                println!("O valor do elemento no índice {} é: {}", index, element);
            } else {
                println!("Índice fora dos limites do array.");
            }
        }
        Err(error) => println!("Erro: {}", error),
    }
}
