use std::io;
use std::cmp::Ordering;

fn main() {


    // takes user input. always comes in as string



    // converting to a number without error handling
    // let number: i32 = number.trim().parse().expect("Must enter an integer!");

    // converting to integer and error catching

    // loops need return values in rust
    let number = loop {
        let mut input = String::new();
        println!("Enter a number.");
        io::stdin().read_line(& mut input).expect("Failed to read line.");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
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
