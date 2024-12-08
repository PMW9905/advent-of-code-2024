use input_read_util::read_file_return_buffer;
use std::env;

const MAX_DIF: i32 = 3;

fn does_level_match_rules(should_increase: &bool, prev_level: &i32, cur_level: &i32) -> bool {
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
        let mut bad_levels = 0;
        let mut ignore_current_level = false;

        for i in 1..report.len() {

            if ignore_current_level {
                ignore_current_level = false;
                continue;
            }

            if !does_level_match_rules(&should_increase, &report[i - 1], &report[i]) {
                bad_levels += 1;
                if bad_levels > 1 || i + 1 >= report.len() {
                    break;
                }

                let should_remove_prev = !does_level_match_rules(&should_increase, &report[i - 1], &report[i + 1]);
                let should_remove_cur = !does_level_match_rules(&should_increase, &report[i], &report[i + 1]);

                if should_remove_prev && should_remove_cur {
                    bad_levels += 1;
                    break;
                }

                if should_remove_prev {
                    continue; 
                }

                if should_remove_cur {
                    ignore_current_level = true;
                    continue;
                }
            }
        }

        if bad_levels <= 1 {
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
        }
    }

    println!("{}!", num_success);
}
