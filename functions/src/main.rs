fn main() {
    println!("Hello World!");
    print_labeled_measurement(45,'g');

    let x = five();
    let plus_one = plus_one(x);
    println!("The value of x  plus one is: {plus_one}");
}



fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// specifying the type of return value
fn five () -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}