#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: String) -> Self {
        Self {
            username,
            email: String::from("placeholder@gmail.com"),
            sign_in_count: 1,
            active: true,
        }
    }
}

fn main() {
    let user = User::new(String::from("daki"));

    println!("{:#?}", user);
}
