struct User {
    active: bool,
    username: String,
    email: String,
    sigh_in_cout: u64,
}

pub fn struct_rust() {
    let user1 = User {
        active: true,
        username: String::from("somerusername123"),
        email: String::from("someone@example.com"),
        sigh_in_cout: 1,
    };

    println!(
        "{} {} {} {}",
        user1.active, user1.username, user1.email, user1.sigh_in_cout
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!(
        "{} {} {} {}",
        user2.active, user2.username, user2.email, user2.sigh_in_cout
    );
}
