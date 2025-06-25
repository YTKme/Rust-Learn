use std::io;
use rand::{self, rng};
use rand::prelude::*;

fn main() {
    println!("Module");

    println!("Standard Library");

    println!("Standard Input");

    // let mut buffer = String::new();
    // println!("Enter a Message: ");
    // let _ = io::stdin().read_line(&mut buffer);
    // println!("You Entered: {buffer}");

    println!("Parse String");

    // let number: i32 = buffer.trim().parse::<i32>().unwrap();
    // println!("Your Number + 1: {}", number + 1);

    println!("Crate");

    let random_number = rand::random::<f64>();
    println!("Random Number: {random_number}");

    let number = rng().random_range(1..11);
    println!("Thread RNG: {:?}", number);

    println!("Challenge");
}
