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

// #[derive(Debug)]

struct File {
    name: String,
    data: Vec<u8>,
}

fn memory_arrangement_of_structure() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    // println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black.R is {}", black.0);
    println!("origin.x is {}", origin.0);
}

// #[derive(Debug)]

// struct AlwaysEqual;

fn _unit_like_struct() {
    // let subject = AlwaysEqual;
    // impl SomeTrait for AlwaysEqual {}
    // println!("{:?}", subject);
}

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn print_struct_information() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!("width: {}, height: {}", rect1.width, rect1.height);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
}

fn main() {
    instantiate_struct();
    build_user(
        String::from("constructuser@example.com"),
        String::from("constructusername"),
    );
    memory_arrangement_of_structure();
    tuple_struct();
    // unit_like_struct();
    print_struct_information();
}
