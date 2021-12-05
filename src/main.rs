use std::io;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Memory {
    l1: Vec<f32>,
    l2: Vec<f32>,
    l3: Vec<f32>,
    l4: Vec<f32>,
    l5: Vec<f32>,
    l6: Vec<f32>
}

fn main() {
    let mut memory: Memory;
    println!("Read saved memory?");
    let mut response = String::new();
    io::stdin().read_line(&mut response)
        .expect("Error: Failed to read line. main() reading memory.");
    memory = match response.to_lowercase().trim() {
        "yes" => {
            let data = fs::read_to_string("src/memory.json")
                .expect("Error: Failed to read file.");
            let result: Memory = serde_json::from_str(&data).unwrap();
            result
        },
        _ => Memory {
            l1: Vec::new(),
            l2: Vec::new(),
            l3: Vec::new(),
            l4: Vec::new(),
            l5: Vec::new(),
            l6: Vec::new()
        }
    };
    
    println!("{:#?}", memory);
    loop {
        let result = stat(&mut memory);
        match result.as_str() {
            "exit" => break,
            _ => (),
        }
    }
    println!("Save calculator memory? YES | NO");
    let mut response = String::new();
    io::stdin().read_line(&mut response)
        .expect("Error: Failed to read line. main() saving memory.");
    match response.to_lowercase().trim() {
        "yes" => {
            let json = serde_json::json!(memory).to_string();
            let file = fs::OpenOptions::new().write(true).truncate(true)
                .open("src/memory.json");
            let mut file = match file {
                Ok(file) => file,
                Err(error) => panic!("Error: Failed to open file.")
            };
            file.write_all(&json.as_bytes());
            println!("Memory saved!");
        },
        _ => println!("Omitting save...")
    }
    println!("Until next time!");
}

fn stat(memory: &mut Memory) -> String {
    let mut command = String::new();
    println!("STAT: EDIT | CALC | EXIT");
    io::stdin().read_line(&mut command)
        .expect("Error: Failed to read line. stat()");
    match command.to_lowercase().trim() {
        "edit" => edit(memory),
        "calc" => calc(memory),
        "exit" => return "exit".to_string(),
        _ => println!("Please enter a valid command."),
    }
    return "".to_string();
}

fn edit(memory: &mut Memory) {
    println!("EDIT LIST:");
    edit_list(ask_list(memory));
}

fn edit_list(list: &mut Vec<f32>) {
    println!("Input all elements of the list.");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Error: Failed to read line. edit_list()");
    let mut seperator;
    println!("What is the delimiter for the list? SPACE | COMMA | COMMA SPACE | NEW LINE | CUSTOM");
    let mut response = String::new();
    io::stdin().read_line(&mut response)
        .expect("Error: Failed to read line. edit_list() delimiter.");
    match response.to_lowercase().trim() {
        "space" => seperator = String::from(" "),
        "comma" => seperator = String::from(","),
        "comma space" => seperator = String::from(", "),
        "new line" => seperator = String::from("\n"),
        "custom" => {
            seperator = String::new();
            println!("Enter delimiter: ");
            io::stdin().read_line(&mut seperator)
                .expect("Error: Failed to read line. edit_list() custom delimiter.");
            seperator = seperator.trim().to_string();
        },
        _ => {
            println!("Invalid response.");
            return;
        }
    }
    
    let string_vec: Vec<&str> = input.trim().split(&seperator).collect();
    for string in string_vec {
        match string.parse() {
            Ok(num) => list.push(num),
            Err(_) => {
                println!("Error parsing string: {}", string);
            }
        };
    }
    println!("{:?}", &list);
}

fn calc(memory: &Memory) {
fn ask_list(memory: &mut Memory) -> &mut Vec<f32> {
    loop {
        let mut command = String::new();
        println!("L1 L2 L3 L4 L5 L6");
        io::stdin().read_line(&mut command)
            .expect("Error: Failed to read line. ask_list()");
        let list = match command.to_lowercase().trim() {
            "l1" => &mut memory.l1,
            "l2" => &mut memory.l2,
            "l3" => &mut memory.l3,
            "l4" => &mut memory.l4,
            "l5" => &mut memory.l5,
            "l6" => &mut memory.l6,
            _ => {
                println!("Invalid response. Try again.");
                continue;
            }
        };
        return list;
    }
}

// Implementation that does not return a mutable list
fn ask_list_ref(memory: &Memory) -> &Vec<f32> {
    loop {
        let mut command = String::new();
        println!("L1 L2 L3 L4 L5 L6");
        io::stdin().read_line(&mut command)
            .expect("Error: Failed to read line. _ref()");
        let list = match command.to_lowercase().trim() {
            "l1" => &memory.l1,
            "l2" => &memory.l2,
            "l3" => &memory.l3,
            "l4" => &memory.l4,
            "l5" => &memory.l5,
            "l6" => &memory.l6,
            _ => {
                println!("Invalid response. Try again.");
                continue;
            }
        };
        return list;
    }
}