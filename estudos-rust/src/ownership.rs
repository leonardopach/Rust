pub fn ownership() {
    println!("hello world");
    let s = "ola";
    println!("{s}");

    {
        let _s = "texto";
    }
    let mut s = String::from("texto");

    println!("{s}");

    s.push_str(", mundo");
    println!("{s}");

    let s1 = String::from("Texto");
    let s2 = s1.clone();

    println!("s1={s1} s2={s2}");

    let s = String::from("texto");

    toma_posse(s);

    let x = 5;

    faz_uma_copia(x);
    println!("{x}");

    let s1 = String::from("texto");

    let (s2, tamanho) = calcula_tamanho(s1);

    println!("O tamanho de '{}' e {}.", s2, tamanho);
}

fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len();

    (s, tamanho)
}

fn toma_posse(uma_string: String) {
    println!("{uma_string}");
}

fn faz_uma_copia(um_inteiro: i32) {
    println!("{}", um_inteiro);
}
