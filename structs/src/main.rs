struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!(
        "{}, {}, {}, {}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!(
        "{}, {}, {}, {}",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );

    let user3 = User {
        email: String::from("another3@example.com"),
        username: String::from("anotherusername5678"),
        ..user1
    };
    println!(
        "{}, {}, {}, {}",
        user3.email, user3.username, user3.active, user3.sign_in_count
    );

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", origin.0, origin.1, origin.2);
}
