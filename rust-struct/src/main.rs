struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn instantiate_struct() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: user1.username, //ownership transfer, moved value
        active: user1.active,
        sign_in_count: user1.sign_in_count,
        // ..user1
    };

    println!("{}", user1.active);
    println!("{}", user1.email); //impl the struct
    println!("{}", user2.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand struct initialization
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    instantiate_struct();
    build_user(
        String::from("constructuser@example.com"),
        String::from("constructusername"),
    );
}
