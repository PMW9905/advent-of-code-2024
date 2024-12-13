use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Mul;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Error: please supply a path to file and the desired number of blinks.");
        return;
    }
    let file_path: String = String::from(args[1].clone());
    let Ok(input_file) = File::open(file_path) else {
        println!("Error opening file");
        return;
    };
    println!("File opened successfully");

    let reader = BufReader::new(input_file);
    let lines = reader.lines();

    for line in lines {
        match line {
            Ok(initial_stone_set) => {
                // let mut result = initial_stone_set.len();
                let parsed_stones = initial_stone_set
                    .split_whitespace()
                    .map(|split| split.to_string())
                    .collect::<Vec<String>>();

                let mut result = 0;
                let blinks: usize = args[2].parse::<usize>().unwrap();
                let mut cache: HashMap<(String, usize), i64> = HashMap::new();

                for stone in parsed_stones {
                    result += shift_stone_through_blinks(stone, blinks, &mut cache);
                }
                println!("Result after {} blinks: {}", blinks, result)
            }
            _ => {}
        }
    }
}

/*
    Create a recursive algo for the stones
*/
fn shift_stone_through_blinks(
    value: String,
    remaining_blinks: usize,
    cache: &mut HashMap<(String, usize), i64>,
) -> i64 {
    let mut result: i64 = 0;
    // base case
    if remaining_blinks == 0 {
        result += 1
    } else {
        let value_magnitude = value.len();
        let next_itr_blinks = remaining_blinks - 1;

        // if there is a value in the cache with same number and amount of blinks, return that to save calculation time
        // else determine value
        match cache.get(&(value.clone(), remaining_blinks)) {
            // if this number has been
            Some(cached_result) => return cached_result.clone(),
            None => {
                if value == "0" {
                    let new_value = String::from("1");
                    result += shift_stone_through_blinks(new_value, next_itr_blinks, cache);
                } else if value_magnitude % 2 == 0 {
                    let mut first_half = value.clone();
                    let second_half = first_half
                        .split_off(value_magnitude / 2)
                        .parse::<i64>()
                        .unwrap()
                        .to_string();

                    result += shift_stone_through_blinks(first_half, next_itr_blinks, cache);
                    result += shift_stone_through_blinks(second_half, next_itr_blinks, cache);
                } else {
                    let new_value = value.clone().parse::<i64>().unwrap().mul(2024).to_string();
                    result += shift_stone_through_blinks(new_value, next_itr_blinks, cache);
                }
                cache.insert((value, remaining_blinks), result);
            }
        }
    }

    return result;
}
