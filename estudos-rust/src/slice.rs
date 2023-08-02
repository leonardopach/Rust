pub fn slice() {
    let mut s = String::from("Meu primeiro nome");

    let palavra = primeira_palavra(&s);

    println!("{palavra}");
    s.clear();

    println!("{s}");

    let s = String::from("Texto longo");

    let texto = &s[0..5];
    let longo = &s[6..11];

    println!("{texto}");
    println!("{longo}");

    let a = [1, 2, 3, 4, 5, 5];

    let slice = &a[1..3];

    println!("{:?}", slice);
}

fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
