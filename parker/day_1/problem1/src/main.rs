use input_read_util::read_file_return_buffer;
use std::env;

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
    let mut right_nums: Vec<i32> = Vec::new();

    for line in input {
        let mut line_whitespace = line.split_whitespace(); 
        let left_num: i32 = line_whitespace.next().unwrap_or_default().parse().expect("Unable to parse left of {line}");
        let right_num: i32 = line_whitespace.next().unwrap_or_default().parse().expect("Unable to parse right of {line}");

        left_nums.push(left_num);
        right_nums.push(right_num);
    }

    left_nums.sort();
    right_nums.sort();

    let mut dif: i32 = 0;

    for (a, b) in left_nums.iter().zip(right_nums.iter()) {
        dif += (a - b).abs();
    }

    println!("{}",dif);
}
