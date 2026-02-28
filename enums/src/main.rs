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

    /// option enum
    let some_number = Some(5);  // the type of this variable is also Option<i32>
    let some_char = Some('g'); // the type of this variable is Option<char> - rust knowns the type becasue we specified the value in the Some variant

    let absent_number: Option<i32> = None; // this variable can be a number or no value - Rust cant infer the value type so we have to declare it in <i32>


    let ab: i8 = 5;
    let cd: Option<i8> = 6;

    let ef = ab + cd;
     // This wont compile because you can't add a possible null value to an i8. Both values have to definitely be valid
}

//fn route(IpAddr: IpAddrKind) { // function takes either type of the IPAddrKind enum
//}