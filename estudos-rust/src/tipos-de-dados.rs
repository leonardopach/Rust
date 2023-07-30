pub fn tipos_de_dados() {
    let guess: u32 = "42".parse().expect("Não é um número");
    println!("{guess}");

    let valor_inteiro: i32 = -10;
    let valor_inteiro_sem_sinal: u32 = 10;
    let valor_flutuante: f32 = 10.2;

    println!("{valor_inteiro}, {valor_inteiro_sem_sinal}, {valor_flutuante}");

    let t: bool = true;
    println!("{t}");

    let c = 'z';
    println!("{c}");

    // tupla
    let tup: (i32, f64, u8) = (500, 12.4, 9);
    println!("{:?}", tup);
    println!("{}", tup.0);

    // lista
    let a = [1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", a);
    println!("{}", a[a.len() - 1]);
}
