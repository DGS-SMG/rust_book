fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else if number > 8 {
        println!("2nd soncidtion was true");
    } else {
        println!("Condition was false");
    }

    let condition = true;

    let new_number = if condition { 5 } else { 6 }; // all possible returned values from an if block must be the same type
    println!("The value of the new number is: {new_number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 5 {
            break counter * 2;
        }
    };

    println!("Results is {result}");


    // loop labels

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;

        }
        count += 1;
    }
    println!("End count = {count}");


    let mut nm_num = 3;

    while nm_num != 0 {
        println!("{nm_num}!");
        nm_num -= 1;
    }


    let a  = [2,3,4,56,6,5];
    let mut index = 0;
    while index < 5 { // not an optimal looop for an array as you need to know the lenght of the array
        println!("Index value is {}", a[index]);
        index +=1;
    }

    // better to use for loop instead
    for element in a {
         println!("the value is: {element}");
    }
    

}
