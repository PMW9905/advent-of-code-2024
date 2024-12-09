use input_read_util::read_file_return_buffer;
use std::env;
use regex::Regex;

const SIZE_OF_DO: usize = 4;
const SIZE_OF_DONT: usize = 7;

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

    let do_dont_mul_regex = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]+,[0-9]+\)").unwrap();
    let num_regex = Regex::new(r"[0-9]+").unwrap();

    let mut sum_of_mul = 0; 

    let mut can_mul = true;

    for line in input { 
        for mul in do_dont_mul_regex.find_iter(&line.as_str()) {

            if mul.as_str().len() == SIZE_OF_DO {
                can_mul = true; 
                continue;
            } 

            if mul.as_str().len() == SIZE_OF_DONT {
                can_mul = false;
                continue;
            }

            if !can_mul {
                continue;
            }

            let mut num_iter = num_regex.find_iter(mul.as_str());
            let num1: i32 = match num_iter.next() {
                Some(num) => num.as_str().parse::<i32>().unwrap_or_default(),
                None => {
                    println!("Unable to get next next number in mul func");
                    std::process::exit(1);
                }
            };
            let num2: i32 = match num_iter.next() {
                Some(num) => num.as_str().parse::<i32>().unwrap_or_default(),
                None => {
                    println!("Unable to get next next number in mul func");
                    std::process::exit(1);
                }
            };

           sum_of_mul += num1 * num2; 
        }
    }

    println!("{}", sum_of_mul);
}
