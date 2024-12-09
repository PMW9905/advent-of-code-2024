use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: please supply a path to file.");
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
            Ok(disk_map) => orchetrate_disk_cleanup(disk_map),
            _ => {}
        }
    }
}

fn orchetrate_disk_cleanup(disk_map: String) {
    let (expanded_map, file_disk_space) = expand_disk_map(disk_map.clone());

    let reordered_map = re_order_disk(expanded_map, file_disk_space);

    let sum = checksum(reordered_map);

    println!("sum: {}", sum)
}

// takes compact disk map and expands values into better representations of what it meansturns it into format -> ##.....#####...##..#.##...
fn expand_disk_map(disk_map: String) -> (Vec<String>, i32) {
    let parsed_nums = disk_map
        .chars()
        .map(|char| char.to_string())
        // .filter_map(|char| char.to_digit(10))
        .collect::<Vec<String>>();

    // replace every even index (or 0-index) with a number of chars (whose value is the file's 0-index by order of appearence)

    // replace each odd index with a number of '.' equal to the value of the source char

    let mut expanded_disk_map = vec![];
    let mut total_file_disk_space = 0;
    for (i, num) in parsed_nums.iter().enumerate() {
        let mut expanded_representation = match i % 2 {
            0 => {
                let Ok(parsed_num) = num.parse::<i32>() else {
                    // TODO: expand this error handling
                    continue;
                };

                let file_index: usize = i / 2;

                total_file_disk_space += parsed_num;
                vec![file_index.to_string(); parsed_num as usize]
            }
            1 => {
                let Ok(parsed_num) = num.parse::<i32>() else {
                    // TODO: expand this error handling
                    continue;
                };

                vec![String::from("."); parsed_num as usize]
            }
            _ => vec![],
        };

        expanded_disk_map.append(&mut expanded_representation);
    }

    (expanded_disk_map, total_file_disk_space)
}

/*
   Algorithm:
   - keep a vecdeque for all numbers of format VecDeque<(source_arr_index, char_value)>
   - pop_back from numbers, pop_front for free space
   - swap popped values,
   - repeat until all free spaces are contiguous
*/
fn re_order_disk(expanded_map: Vec<String>, total_file_disk_space: i32) -> Vec<String> {
    let mut empty_disk_space: VecDeque<usize> = VecDeque::new();
    let mut reversed_file_locations: VecDeque<usize> = VecDeque::new();

    for (i, slot_value) in expanded_map.iter().enumerate() {
        let is_empty_space = slot_value.contains(".");
        if is_empty_space {
            // we do not care about free space that occurs after the point where, once sorted, all file data will be stored
            if i < total_file_disk_space as usize {
                empty_disk_space.push_back(i);
            }
        } else {
            reversed_file_locations.push_front(i);
        }
    }

    let mut map_clone = expanded_map.clone();
    while !empty_disk_space.is_empty() {
        let first_open_disk_slot = empty_disk_space.pop_front().unwrap();
        let last_file_disk_slot = reversed_file_locations.pop_front().unwrap();

        map_clone.swap(last_file_disk_slot, first_open_disk_slot);
    }

    map_clone
}

fn checksum(disk_map: Vec<String>) -> i64 {
    let mut sum: i64 = 0;
    for (i, char) in disk_map.iter().enumerate() {
        let Ok(parsed_char) = char.parse::<i64>() else {
            continue;
        };

        sum += parsed_char * (i as i64)
    }

    return sum;
}
