fn statementsAdnExpressions() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = { 2 * x };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    let v = {
        let x = 1;
        x + 2
    };

    assert_eq!(v, 3);

    let v = 3;

    assert!(v == 3);

    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
