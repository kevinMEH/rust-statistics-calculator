use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        stat();
    }
    println!("Until next time!");
}

fn stat() {
    let mut command = String::new();
    println!("STAT: EDIT | CALC | EXIT");
    io::stdin().read_line(&mut command)
        .expect("Error: Failed to read line.");
    match command.to_lowercase() {
        "edit" => edit(),
        "calc" => calc(),
        "exit" => return,
        _ => println!("Please enter a valid command."),
    }
}

fn edit() {
    
}