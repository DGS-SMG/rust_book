enum IpAddrKind {
    V4(u8, u8, u8, u8), // still not storing actual ip address data, just the type - data can be added to each enum variant in brackent
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32,i32)
}
// you can put any type of data inside an enum variant, structs, numbers, strings, booleans,, tuples

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959
        }
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState) // this variant includes data of type enum UsState
}

impl Message {
    fn call (&self) {
    }
}
// no longer needed when we add data to each enum variant
struct IpAddr { // not the best way to store the address witht he enum type
    kind: IpAddrKind,
    address: String
}

//let home: IpAddr { 
//    kind: IPAddrKind::v4,
//    address: String::from("127.0.0.1")
//}
//let loopback: IpAddr {
//    kind: IpAddrKind::v6,
//    address: String::from("::1"),
//}



// Option enum





fn main() {
    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;

    let home = IpAddrKind::V4(127,0,0,1); // mutliple data can be stored within each enum type - see how this is storing 4 u8 numbers whilst v6 is storing a string still - this wouldnt be possible with a struct

    let loopback = IpAddrKind::V6(String::from("::1"));

    //route(IPAddrKind::v4);
    //route(IPAddrKind:v6);

    let m = Message::Write(String::from("Hello my friend"));
    m.call();

    // option enum
    let some_number = Some(5);  // the type of this variable is also Option<i32>
    let some_char = Some('g'); // the type of this variable is Option<char> - rust knowns the type becasue we specified the value in the Some variant

    let absent_number: Option<i32> = None; // this variable can be a number or no value - Rust cant infer the value type so we have to declare it in <i32>


    let ab: i8 = 5;
    //let cd: Option<i8> = 6;

    //let ef = ab + cd;
     // This wont compile because you can't add a possible null value to an i8. Both values have to definitely be valid

    // match expressions

    value_in_cents(Coin::Quarter(UsState::Alaska));


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    // catch all logic in a match function

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(), // use literal values in math expressiom
        7 => remove_fancy_hat(),
        other => move_player(other) // other covers everthing else and passes value to move_player function
        // the other catch all pattern makes this match expression exhaustive
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll() // _ is used as a catch all when we dont want to catch the actual value but want to execute some code
        // this also meets the exhuastive requirement
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => () // can also execute no code in the catch all function
    }

    // example using match just wiht one expression
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => ()
    }

    // let if logic - allow you to match one pattern but ignore the rest. this is shorter to tye but loses the exhuastive property of match
    let config_max_2 = Some(3u8);
    if let Some(max) = config_max_2 { // max binds to the value inside Some - which is tested against the argument on the right of the = sign
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    // this match expression can be relaces with if let and else

    let coin2 = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin2 {
        println!("State qurter from {state:?}");
    } else {
        count += 1;
    }


}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // with match the condition can evaluate to any value, where as an if statement requires the statement to evaluate to a bool
        Coin::Penny => 1,
        Coin::Nickel => 5, // each of these lines is a match arm
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter is {state:?}");
            println!("Lucky twenty five!"); // multi line arms require curly brackets to execute code
            25
        }
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {  // match expression must have arm to cover all possibilities. will return error - non-exhaustive patterns: `None` not covered
        None => None,
        Some(i) => Some(i + 1)  // value of Some is bound to i 
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}



//fn route(IpAddr: IpAddrKind) { // function takes either type of the IPAddrKind enum
//}

// if let 

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for here :)"))
        } else {
            Some(format!("{state:?} is relatively new"))
        }
    } else {
        None
    }
}

fn describe_state_quarter_refactor(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old ...."))
    } else {
        Some(format!("{state:?} is new..."))
    }
}

fn describe_state_quarter_else(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1900){
        Some(format!("{state:?} is old my mannnn"))
    } else {
        Some(format!("{state:?} is not so old my guuuy"))
    }
}

fn describe_state_quarter_match(coin: Coin) -> Option<String> {
    match coin {
        Coin::Quarter(state) => {
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old..."))
            } else {
                Some(format!("{state:?} is not so old... "))
            }
        },
        _ => None
    }
}