pub fn refereces_and_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
    println!("{s1}");

    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");
    let a = [1, 2, 3, 4];

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    println!("{:?}", r1);
    println!("{a:?}");

    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}
