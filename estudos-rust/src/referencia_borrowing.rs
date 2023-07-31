fn referencia_borrowing() {
    let s1 = String::from("Texto");

    let tamanho = calcula_tamanho(&s1);

    println!("O tamanho de '{}' é {}.", s1, tamanho);

    let mut s = String::from("Text");
    modifica(&mut s);

    print!("{s}");

    let referencia_para_o_nada = soltar();
}

fn soltar() -> &String {
    let s = String::from("Texto");

    s
}

fn modifica(uma_string: &mut String) {
    uma_string.push_str(" longo")
}
fn calcula_tamanho(s: &String) -> usize {
    s.len()
}
