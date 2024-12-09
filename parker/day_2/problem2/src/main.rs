use input_read_util::read_file_return_buffer;
use std::env;

const MAX_DIF: i32 = 3;

fn does_level_match_rules(should_increase: &bool, prev_level: &i32, cur_level: &i32) -> bool {

    println!("{} {} {} {}", *prev_level, *cur_level, prev_level, cur_level);

    if *prev_level == *cur_level {
        return false;
    }

    if (*prev_level < *cur_level) != *should_increase {
        return false;
    }

    if (*prev_level - *cur_level).abs() > MAX_DIF {
        return false;
    }

    return true;
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    if report.len() <= 1 {
        return true;
    }


    for should_increase in vec![true, false] {
        let mut errors_found: u32 = 0;
        let mut skip_this_level = false;
        for i in 1..report.len() {

            if skip_this_level {
                skip_this_level = false;
                continue;
            }

            if !does_level_match_rules(&should_increase, &report[i - 1], &report[i]) {
                println!("{}",i);
                errors_found += 1;

                if errors_found > 1 || i + 1 == report.len() {
                    break;
                } 

                let prev_can_be_removed = i-1 == 0 || does_level_match_rules(&should_increase, &report[i-2], &report[i]); 
                let cur_can_be_removed = i+1 >= report.len() || does_level_match_rules(&should_increase, &report[i-1], &report[i+1]); 
                println!("prev remove? : {}, cur remove? : {}",prev_can_be_removed,cur_can_be_removed);

                if cur_can_be_removed {
                    skip_this_level = true;
                    continue;
                }

                if prev_can_be_removed {
                    continue;
                }

                if !prev_can_be_removed && !cur_can_be_removed {
                    errors_found+=1;
                    break;
                }
            }
        }
        if errors_found <= 1 {
            return true;
        }
    }
    return false;
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
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if is_report_safe(&report) {
            num_success += 1;
            println!("{:?}", &report);
        }
    }

    println!("{}!", num_success);
}
