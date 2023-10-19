fn shadowing() {
    // shading
    let x: i32 = 5;

    {
        let x = 12;
        assert_eq!(x, 12);
        println!("success {}", x);
    }

    assert_eq!(x, 5);
    println!("success {}", x);
    let x: i32 = 42;
    println!("{}", x);

    let mut x: i32 = 1;
    println!("{}", x);
    x = 7;
    println!("{x}");
    let mut x = x;
    x += 3;
    println!("{x}");

    let y: i32 = 4;
    println!("{}", y);
    let y: &str = "I can also be bound to text!";
    println!("{}", y);

    println!("Success");
}
