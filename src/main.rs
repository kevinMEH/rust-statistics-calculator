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
    // Initialize memory
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
    
    

    // Calculator Loop
    loop {
        let result = stat(&mut memory);
        match result.as_str() {
            "exit" => break,
            _ => (),
        }
    }
    
    

    // Saving memory
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
                Err(error) => panic!("Error: Failed to open file. {}", error)
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
    let list: &mut Vec<f32> = list;
    
    println!("Input all elements of the list.");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Error: Failed to read line. edit_list()");
    
    println!("What is the delimiter for the list? SPACE | COMMA | COMMA SPACE | NEW LINE | CUSTOM");
    let mut response = String::new();
    io::stdin().read_line(&mut response)
        .expect("Error: Failed to read line. edit_list() delimiter.");
        
    let seperator = match response.to_lowercase().trim() {
        "space" => String::from(" "),
        "comma" => String::from(","),
        "comma space" => String::from(", "),
        "new line" => String::from("\n"),
        "custom" => {
            let mut temp = String::new();
            println!("Enter delimiter: ");
            io::stdin().read_line(&mut temp)
                .expect("Error: Failed to read line. edit_list() custom delimiter.");
            
            temp.trim().to_string()
        },
        _ => {
            println!("Invalid response.");
            return;
        }
    };
    
    let string_vec: Vec<&str> = input.trim().split(&seperator).collect();
    
    println!("APPEND or REPLACE?");
    let mut response = String::new();
    io::stdin().read_line(&mut response)
        .expect("Error: Failed to read line. APPEND or REPLACE.");
    match response.to_string().trim() {
        "append" => (),
        "replace" => list.clear(),
        _ => {
            println!("Invalid response.");
            return;
        }
    };

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
    println!("1VSTATS | 2VSTATS | LINREG");
    let mut response = String::new();
    io::stdin().read_line(&mut response)
        .expect("Error: Failed to read line. calc()");
    match response.to_lowercase().trim() {
        "1vstats" => one_v_stats(memory),
        "2vstats" => two_v_stats(memory),
        "linreg" => linreg(memory),
        _ => println!("Invalid response.")
    }
}


// 0, 1, 2, 3, 4, 5, 6         7
// 0, 1, 2, 3, 4, 5, 6, 7      8
fn one_v_stats(memory: &Memory) {
    println!("Select list:");
    
    let mut list: Vec<f32> = ask_list_ref(&memory).clone();
    quickersort::sort_floats(&mut list[..]);
    if list.len() == 0 {
        println!("Empty list.");
        return;
    }
    let mean: f32;
    let mut sum: f32 = 0.0;
    let mut squared_sum: f32 = 0.0;
    let sample_std: f32;
    let population_std: f32;
    let n = list.len();
    let min = list[0];
    
    if list.len() < 4 {
        println!("List length is less than 4, quartile calculation skipped.");
    }
    
    let q1: f32 = {
        // Honest cannot think of a better way so whatever
        if list.len() < 4 {
            0.0
        } else {
            
            let list = &list[.. list.len() / 2 - 1]; // Lower list
            // If you're wondering why I'm comparing to 0 and then matching instead of just matching 0 and 1
            // Rust doesn't recognize that % 2 will only return 2 possible values: 0 and 1, and it wants you
            // to include a default clause even though it's not needed. Going to make it a boolean to avoid
            // adding a default clause.
            match list.len() % 2 == 0 {
                true => (list[list.len() / 2 - 1] + list[list.len() / 2]) / 2.0,
                false => list[list.len() / 2]
            }
            
        }
    };
    let median: f32 = match list.len() % 2 == 0 {
        true => (list[list.len() / 2 - 1] + list[list.len() / 2]) / 2.0,
        false => list[list.len() / 2]
    };
    let q3: f32 = {
        if list.len() < 4 {
            0.0
        } else {
        
            let list = &list[(list.len() + 1) / 2 ..]; // Upper list
            match list.len() % 2 == 0 {
                true => (list[list.len() / 2 - 1] + list[list.len() / 2]) / 2.0,
                false => list[list.len() / 2]
            }
             
        }
    };
    let max = list[list.len() - 1];
    
    for num in list.iter() {
        sum = sum + num;
        squared_sum = squared_sum + num * num;
    }
    mean = sum / n as i32 as f32;
    
    let mut summation: f32 = 0.0;
    for num in list.iter() {
        summation = summation + (num - mean) * (num - mean);
    }
    sample_std = (summation / ((n - 1) as i32 as f32)).sqrt();
    population_std = (summation / (n as i32 as f32)).sqrt();
    
    println!("Mean: {}", mean);
    println!("Sum: {}", sum);
    println!("Sum^2: {}", squared_sum);
    println!("Sample Standard Deviation: {}", sample_std);
    println!("Population Standard Deviation: {}", population_std);
    println!("Size: {}", n);
    println!("Min: {}", min);
    if list.len() > 4 {
        println!("Q1: {}", q1);
    }
    println!("Median: {}", median);
    if list.len() > 4 {
        println!("Q3: {}", q3);
    }
    println!("Max: {}", max);
}


// 
fn two_v_stats(memory: &Memory) {
    
}


fn linreg(memory: &Memory) {
    println!("Select list 1:");
    let list1: &Vec<f32> = ask_list_ref(&memory);
    println!("Select list 2:");
    let list2: &Vec<f32> = ask_list_ref(&memory);
    
    if list1.len() != list2.len() {
        println!("The lists are not the same size.");
        return;
    }
    
    let n = list1.len() as i32 as f32;
    let mut sum_x: f32 = 0.0;
    let mut sum_y: f32 = 0.0;
    let mut sum_xy: f32 = 0.0;
    let mut sum_xx: f32 = 0.0; // Bad naming, might change later
    let mut sum_yy: f32 = 0.0;
    
    for (index, x) in list1.iter().enumerate() {
        let y = list2[index];
        sum_x = sum_x + x;
        sum_xx = sum_xx + x * x;
        sum_y = sum_y + y;
        sum_yy = sum_yy + y * y;
        sum_xy = sum_xy + x * y;
    }
    
    // m = ( n(sum(xy)) - sum(x)sum(y) ) / ( n(sum(x^2)) - sum(x)^2 )
    let slope = ( n * sum_xy - sum_x * sum_y) / ( n * sum_xx - sum_x * sum_x);
    
    // b = ( sum(y) - m(sum(x)) ) / n
    let intercept = ( sum_y - slope * sum_x ) / n;
    
    // r = ( n(sum(xy)) - sum(x)sum(y) ) / sqrt( ( n(sum(x^2)) - sum(x)^2 ) ( (n(sum(y^2)) - sum(y)^2 ) ) )
    let r: f32 = ( n * sum_xy - sum_x * sum_y) / ( ( n * sum_xx - sum_x * sum_x) * ( n * sum_yy - sum_y * sum_y) ).sqrt();
    
    let rr = r * r; // Terrible naming, but no ideas at the moment
    
    println!("y = {}x + {}", slope, intercept);
    println!("r = {}", r);
    println!("r^2 = {}", rr);
}

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