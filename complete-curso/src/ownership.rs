pub fn ownership() {
    let s = "hello, my name is Leonardo";

    println!("{s}");

    let mut s = String::from("hello");
    println!("{s}");
    s.push_str(", world");
    println!("{s}");

    {
        let y = String::from("hello");
        println!("{y}");
    }

    let x = 5;
    let y = x;

    println!("{}, {y}", x);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", &s1, &s2);

    let s = String::from("hello");

    takes_ownership(s);
    // println!("{s}"); // takes ownership

    let x = 5;
    makes_copy(x);

    println!("{x}");
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{s1} {s3}");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn makes_copy(x: i32) {
    println!("{} inside  makes_copy", x)
}
fn takes_ownership(s: String) {
    println!("{} inside takes_ownership", s)
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
