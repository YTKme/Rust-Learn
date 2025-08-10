// use std::env;
use std::{env, fs};
use std::io::prelude::*;

fn main() {
    // Reading From Command Line
    // if env::args().len() < 3 {
    //     println!("Please provide at least two arguments.");
    //     return;
    // }

    // for (index, argument) in env::args().enumerate() {
    //     println!("Argument {index}: {argument}");
    // }

    // let arg2 = env::args().nth(2).unwrap();
    // println!("The second argument is: {arg2}");

    // Reading From File
    // let content = fs::read_to_string("planet.txt").unwrap();
    // print!("File Content:\n{content}");
    // for line in content.lines() {
    //     println!("Line: {line}");
    // }
    // Read Byte
    // let content = fs::read("planet.txt").unwrap();
    // print!("File Content:\n{content:?}");

    // Write To File
    // let mut speech = String::new();
    // speech.push_str("We choose to go to the Moon in this decade\n");
    // speech.push_str("and do the other things,\n");
    // speech.push_str("not because they are easy,\n");
    // speech.push_str("but because they are hard.\n");

    // let _ = fs::write("speech.txt", speech);

    // let mut file = fs::OpenOptions::new()
    //     .append(true)
    //     .open("planet.txt")
    //     .unwrap();

    // let _ = file.write(b"\nPluto");

    // Challenge

    if env::args().len() < 3 {
        println!("Please provide at least two arguments.");
        return;
    }

    let filename = env::args().nth(1).unwrap();
    println!("Filename: {filename}");

    let search_name = env::args().nth(2).unwrap();
    println!("Search Name: {search_name}");

    let binding = fs::read_to_string(&filename).unwrap();
    let content = binding.lines();
    for line in content {
        if line.contains(&search_name) {
            println!("{search_name} did walk on the Moon!");
            return;
        }
    }

    print!("{search_name} di NOT walk on the Moon... YET.");

}
