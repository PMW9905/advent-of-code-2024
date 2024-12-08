use input_read_util::read_file_return_buffer;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid number of args");
        println!("problem1 <path_to_input>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    
    let input: Vec<String> = match read_file_return_buffer(input_path) {
        Ok(input) => input,
        Err(error) => {
            println!("Error trying to read input: {}", error);
            std::process::exit(1);
        }
    }; 

    let mut left_nums: Vec<i32> = Vec::new();
    let mut right_nums_map: HashMap<i32,i32> = HashMap::new();

    for line in input {
        let mut line_whitespace = line.split_whitespace(); 
        let left_num: i32 = line_whitespace.next().unwrap_or_default().parse().expect("Unable to parse left of {line}");
        let right_num: i32 = line_whitespace.next().unwrap_or_default().parse().expect("Unable to parse right of {line}");

        left_nums.push(left_num);

        let prev_value: i32 = match right_nums_map.get(&right_num) {
            Some(value) => value.clone(),
            None => 0
        };

        right_nums_map.insert(right_num, prev_value+1);
    }

    let mut dif: i32 = 0;

    for num in left_nums.iter() {
                
        let times_seen: i32 = match right_nums_map.get(&num) {
            Some(value) => value.clone(),
            None => 0
        };

        dif += num * times_seen;
    }

    println!("{}",dif);
}
