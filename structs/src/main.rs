struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // unit struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { // creating a method on Rectangle calling its self as values
    fn area (&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold (&self, rect_to_compare: &Rectangle) -> bool {
        self.width > rect_to_compare.width && self.height > rect_to_compare.height
    }
}
enum IpAddrKind {
    V4(String),
    V6(String)
}
    
fn main() {
    println!("Hello, world!");
    
    let s = String::from("Green potatoes");
    
    let slice = &s[0..5];
    let same_slice = &s[..5];

    println!("{same_slice}");
    
    let len = s.len();
    let last_slice = &s[6..len];
    let same_last_slice = &s[6..];
    println!("{same_last_slice}");
    
    let whole_slice = &s[0..len];
    let same_whole_slice = &s[..]; // drop both values
    println!("{same_whole_slice}");
    
    let a = first_word(&s);
    println!("{a}");
    

    let mut user1 = User {
        active: true,
        username: String::from("domsmg"),
        email: String::from("domsmg@gmail.com"),
        sign_in_count: 1
    };
    
    user1.email = String::from("newemail@gmail.com"); // reassign the value of a struct only when it is mutable
    
    println!("{}", user1.email);
    
    let new_user = build_user(String::from("email@hotmail.com"), String::from("emailsmith")); // built in a function
    
    // copying some elements form one user to another
    let user2 = User {
        active: user1.active,
        username: user1.username, // string values do no implement the copy trait, meaning they are stored on the heap, rendering user1 no longer usable after this call
        email: String::from("newestemail@jher.com"),
        sign_in_count: user1.sign_in_count
    };
    
    let user3 = User {
        email: String::from("bettersyntax@yes.com"),
        ..user2
    };
    
    println!("{}",user1.email); // email value can be used as it hasnt been borrowed
    //println!("{}",user1.username); // username cant be used as it was reassinged to user2.username


    // tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    let Point(x,y,z) = origin; // you can use object destructuring with struct tuples but they need to be named
    
    let subject = AlwaysEqual; // assigned to unit struct
    
    let scale = 2;
    
    let rect = Rectangle {
        width: dbg!(6 * scale),
        height: 4
    };
    
    let rect1 = Rectangle {
        width: 12,
        height: 4
    };
    
    let rect2 = Rectangle {
        width: 6,
        height: 3
    };
    
    let rect3 = Rectangle {
        width: 13,
        height: 3
    };
    
    dbg!(&rect);
    
    
    //println!("rect is {rect:#?}"); // thiswill not work as rust cant print a plan struct. Adding :? will print the debug version, :#? will print the prettier version
    
    //let area = area(rect);
    
    println!("{}",rect.area()); // calling the saem structbut with a dedicated method
    
    println!("Rect 1 can hold rect 2: {}", rect1.can_hold(&rect2));
    println!("Rect 1 can hold rect 3: {}", rect1.can_hold(&rect3));
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); 
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    let len = s.len();
    &s[..]
    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn route(ip:IpAddrKind) {
    
}