pub fn slice() {
    let value = String::from("O rato sujou a roupa do rei de roma");

    println!("{}", first_word(&value))
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
