use std::io;

struct Memory {
    l1: Vec<f32>,
    l2: Vec<f32>,
    l3: Vec<f32>,
    l4: Vec<f32>,
    l5: Vec<f32>,
    l6: Vec<f32>
}

fn main() {
    let mut memory = Memory {
        l1: Vec::new(),
        l2: Vec::new(),
        l3: Vec::new(),
        l4: Vec::new(),
        l5: Vec::new(),
        l6: Vec::new()
    };
    loop {
        stat();
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
    let mut command = String::new();
    println!("EDIT: L1 L2 L3 L4 L5 L6");
    io::stdin().read_line(&mut command)
        .expect("Error: Failed to read line. edit()");
    match command.to_lowercase().trim() {
        "l1" => edit_list(&mut memory.l1),
        "l2" => edit_list(&mut memory.l2),
        "l3" => edit_list(&mut memory.l3),
        "l4" => edit_list(&mut memory.l4),
        "l5" => edit_list(&mut memory.l5),
        "l6" => edit_list(&mut memory.l6),
        _ => println!("Invalid response.")
    }
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
    
}