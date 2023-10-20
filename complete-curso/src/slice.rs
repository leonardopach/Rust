pub fn slice() {
    let value = String::from("O rato sujou a roupa do rei de roma");

    println!("{}", first_word(&value));
    println!("{}", first_word1(&value));

    let s = String::from("hello world");

    let hello = &s[0..6];
    let world = &s[6..11];

    println!("{} - {world}", hello);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    println!("{:?}", slice);
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

fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
