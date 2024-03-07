fn first_enum() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    fn route(ip_type: IpAddrKind) {
        println!("{:#?}", ip_type);
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:#?} {}", home.kind, home.address);
    println!("{:#?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:#?} {}", loopback.kind, loopback.address);
    println!("{:#?}", loopback);
}

fn another_enum() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:#?}", home);
    println!("{:#?}", loopback);
}

fn third_enum() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:#?}", home);
    println!("{:#?}", loopback);
}

fn fourth_enum() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("Message fn.");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    println!("{:#?}", m);
    let q = Message::Quit;
    println!("{:#?}", q);
    let move_msg = Message::Move { x: 12, y: 13 };
    println!("{:#?}", move_msg);
    match move_msg {
        Message::Move { x, y } => {
            println!("x: {}, y: {}", x, y);
        }
        _ => {
            println!("This message is not a Move variant.");
        }
    }
    let change_color_msg = Message::ChangeColor(1, 2, 3);
    println!("{:#?}", change_color_msg);
}

fn fifth_enum() {
    #[derive(Debug)]
    enum MyOption<T> {
        Some(T),
        None,
    }

    let some_number = MyOption::Some(5);
    let some_string = MyOption::Some("a string");
    let absent_number: MyOption<i32> = MyOption::None;
    println!("{:#?}", some_number);
    println!("{:#?}", some_string);
    println!("{:#?}", absent_number);
}

fn sixth_enum() {
    let x: i8 = 8;
    let y: Option<i8> = Some(5);

    let sum = match y {
        Some(val) => x + val,
        None => x,
    };
    println!("Sum: {}", sum);
}

fn main() {
    first_enum();
    another_enum();
    third_enum();
    fourth_enum();
    fifth_enum();
    sixth_enum();
}
