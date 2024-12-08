use input_read_util::read_file_return_buffer;
use std::env;

const MAX_DIF: i32 = 3;

fn is_level_valid(level: &Vec<i32>) -> bool {

    return true;
}

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

    let mut num_success: i32 = 0;

    for line in input {
        let level: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        if is_level_valid(&level) {
            num_success+=1;
        }
    }

    println!("{}", num_success);
}

















