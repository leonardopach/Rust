fn five(x: i32) -> i32 {
    x
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn functions() {
    println!("Hello world");
    another_function(12);
    print_labeled_measurement(5, 'h');

    // statements are instructions that perform some action and do not return a value
    let _y = 6;

    // error
    // let x = (let y = 6);

    //Expressions evaluate to a resultant value. Let’s look at some examples.
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
    println!("{}", five(5));
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
