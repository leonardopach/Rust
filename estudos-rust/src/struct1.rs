struct User {
    username: String,
    email: String,
    sigh_in_count: u64,
    activate: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        activate: true,
        sigh_in_count: 1,
    }
}
fn struct1() {
    let mut user1 = User {
        email: String::from("algum@exemplo.com"),
        username: String::from("algumnome123"),
        activate: true,
        sigh_in_count: 1,
    };

    user1.email = String::from("outroemail@exemplo.com");
}
