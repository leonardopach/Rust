fn funcao() {
    println!("{:?}", outra_funcao(5));

    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");
}

fn outra_funcao(x: i32) -> i32 {
    x
}
