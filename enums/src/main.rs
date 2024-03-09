fn first_match() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    fn value_in_cents_v2(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let coin = Coin::Penny;
    println!("Penny: {}", value_in_cents(coin));
    let coin = Coin::Nickel;
    println!("Nickel: {}", value_in_cents(coin));
    let coin = Coin::Dime;
    println!("Dime: {}", value_in_cents(coin));
    let coin = Coin::Quarter;
    println!("Quarter: {}", value_in_cents(coin));
    let coin = Coin::Penny;
    println!("Penny: {}", value_in_cents_v2(coin));
}

fn second_match() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn third_match() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    match six {
        Some(value) => println!("Value: {}", value),
        None => println!("None: {}", "None"),
    }
    let none = plus_one(None);
    match none {
        Some(value) => println!("Value: {}", value),
        None => println!("None: {}", "None"),
    }
}

fn fourch_match() {
    let some_u3_value = 3u8;
    match some_u3_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn main() {
    first_match();
    second_match();
    third_match();
    fourch_match();
}
