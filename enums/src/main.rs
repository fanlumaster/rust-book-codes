fn first_if_let() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}

fn second_if_let() {
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn third_if_let() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }

    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }

    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);

    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);

    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);
}

fn fourth_if_let() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);

    let coin = Coin::Nickel;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }
    println!("count: {}", count);

    let coin = Coin::Dime;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }
    println!("count: {}", count);
}

fn main() {
    first_if_let();
    second_if_let();
    third_if_let();
    fourth_if_let();
}
