use std::io;

fn main() {

    println!("Please enter your target number!");

    let mut target_number = String::new();

    io::stdin()
        .read_line(&mut target_number)
        .expect("Failed to read line");

    let target_number : u32 = target_number.trim().parse().expect("Please type a number");

    println!("=========================================================================================");

    for number in 1..target_number+1{
        if number % 15 == 0 {
            println!("FizzBuzz");
        }else if number % 5 == 0 {
            println!("Buzz");
        }else if number % 3 == 0 {
            println!("Fizz");
        }else {
            println!("{number}")
        }
    }

    println!("=========================================================================================");
}