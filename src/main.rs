use std::io;
use std::cmp::Ordering;

fn main() {


    // takes user input. always comes in as string



    // converting to a number without error handling
    // let number: i32 = number.trim().parse().expect("Must enter an integer!");

    // converting to integer and error catching

    // loops need return values in rust
    let number = loop {
        // creates a new string mutable variable
        let mut input = String::new();
        // tells you what is going on
        println!("Enter a number.");
        // tries to get a number from the user
        io::stdin().read_line(& mut input).expect("Failed to read line.");
        // if you did not supply a good number it tries again
        let input: i32 = match input.trim().parse() {
            // if its ok then it returns it
            Ok(num) => num,
            // otherwise it asks again
            Err(_) => continue,
        };
        // ends the loop and returns the number
        break input;

    };


    // comparing using ifs and else
    // if number < 30{
    //     println!("The number is less than thirty.")
    // }else if number > 30{
    //     println!("The number is grater than thirty.")
    // }else {
    //     println!("The number is equal to thirty.")
    // }

    // comparing using Ordering
    match number.cmp(&30){
        Ordering::Less => println!("Less than thirty."),
        Ordering::Greater => println!("Greater than thirty."),
        Ordering::Equal => println!("Equal to thirty."),
    }
}
