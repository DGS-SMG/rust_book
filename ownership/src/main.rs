fn main() {
    // let string_literal = "Hello, world!";

    // this string can be mutated - required mut identifier
    let mut s = String::from("Hello world!"); // undefined string lenght - manages data on the heap - allocation

    s.push_str(", from Mars :)"); //mutates existing string

    s = String::from("Ahoy"); // reallocion to a new variable drops the old varibale from memory

    let s2 = s; // s is dropped here and only s2 is usable

    let s3 = s2.clone(); // If you want to keep a copy of both, clone method is required



    let x = 5;

    let y = x; // x and y both still availabel here.
    // integers have a known sizze at compile time so are stored entirely on the stack

    println!("{s2}");    
    println!("{s3}");
    println!("{y}");



    let s_test = String::from("hello");  // s comes into scope

    takes_ownership(s_test);             // s's value moves into the function...
                                    // ... and so is no longer valid here


    // println!("{s_test}");       // this is no longer in scope becuase it was passed to the function

    let x_test = 5;                      // x comes into scope

    makes_copy(x_test);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

    println!("{x_test}");       // this can be used after being called ina function because it is an integer type (known size) and have copy trait


    // references and borrowing

    let s1 = String::from("Hello");

    // let len = calculate_length(s1); // s1 variable is being used in a function here so then goes out of scope afterwards


    // instead we pass s1 as a reference using the & sybol.

    // & referes to the value of a reference but does not own it
    // creating a reference is known as borrowing
    let len = calculate_length(&s1);


    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("Hi");

    change(&mut s2); // you cannot alter the value of a borrowed value/ reference unless it is declared as mutable using mut

    // one big restriction ona mutable references is that you can only have one at a time
    
    {
        let r1 = &mut s2;
    } // r1 goes out of scope here, so we can make a new reference with no problems.


    let r2 = &mut s2; //  error[E0499]: cannot borrow `s2` as mutable more than once at a time - when r1 and r2 in the same scope

    

    println!("{r2}");



    let mut s_t = String::from("hello");

    let r1_t = &s_t; // no problem
    let r2_t = &s_t; // no problem - Multiple immutable reference are allowed as no data can be changed when someone is reading them
    
    // adding a function that uses these immutable references will end their scope and allow us to create a mutable reference
    println!("{}, {}", r1_t, r2_t);

    let r3_t = &mut s_t; // BIG PROBLEM -  We cannot have a mutable reference when we aready have an immutable one to the same reference unles we use the immutables refences before hand 


    println!("{r3_t}");


    // let reference_to_nothing = dangle();

    let reference_to_something = dangle_fixed();


    let x = String::from("X marks the spot");
    let mut y = String::from("Y marks the spot");

    let x1 = &x;

    let z1 = &x1;

    let y1 = &mut y;

    println!("X is {x1}, Y is {y1}, Z is {z1}")

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.


fn calculate_length(s: &String) -> usize { // this function takes a reference to a string as a value instead
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


// this function throws an error because it returns a reference value out of scope
//fn dangle() -> &String {
//    let s = String::from("hello");

//   &s // s goes out of scope once this function ends so the reference will point to a dropped value

    // the trick here si to return the strict directly instead of a reference to is

//}

fn dangle_fixed() -> String {  // String value retunred instead of reference to String value

    let s = String::from("Hello");
    s // s is returned as the actual value instead of the reference to the value.
}