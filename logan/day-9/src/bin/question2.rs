use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

// range upper bound is exlcusive
type StorageLocations = VecDeque<Range<usize>>;

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
    let (expanded_map, file_disk_range, freespace_disk_range) = expand_disk_map(disk_map.clone());
    let reordered_map = re_order_disk(expanded_map, file_disk_range, freespace_disk_range);
    let sum = checksum(reordered_map);
    println!("checksum: {}", sum)
}

/*
    returns: (0, 1, 2)
    0 - compact disk map and expands values into better representations of what it meansturns it into format -> ##.....#####...##..#.##...
    1 - VecDeque of contiguous index-ranges for all file clusters
    2 - VecDeque of contiguous index-ranges of free space from return value 0
*/
fn expand_disk_map(disk_map: String) -> (Vec<String>, StorageLocations, StorageLocations) {
    let parsed_nums = disk_map
        .chars()
        .map(|char| char.to_string())
        .collect::<Vec<String>>();

    let mut expanded_disk_map = vec![];
    /*
       File storage data:
       - value of page?
       - start and end index
    */
    let mut file_locations: StorageLocations = VecDeque::new();
    let mut free_space_locations: StorageLocations = VecDeque::new();
    for (i, num) in parsed_nums.iter().enumerate() {
        let mut expanded_representation = match i % 2 {
            // replace every even index (or 0-index) with a number of chars (whose value is the file's 0-index by order of appearence)
            0 => {
                let Ok(parsed_num) = num.parse::<i32>() else {
                    // TODO: expand this error handling
                    continue;
                };

                let file_index: usize = i / 2;

                let expanded_index = expanded_disk_map.len();
                // range upper bound is exlcusive
                file_locations.push_front(Range {
                    start: expanded_index,
                    end: expanded_index + (parsed_num as usize),
                });

                vec![file_index.to_string(); parsed_num as usize]
            }
            // replace each odd index with a number of '.' equal to the value of the source char
            1 => {
                let Ok(parsed_num) = num.parse::<i32>() else {
                    // TODO: expand this error handling
                    continue;
                };

                let expanded_index = expanded_disk_map.len();
                // range upper bound is exlcusive
                free_space_locations.push_back(Range {
                    start: expanded_index,
                    end: expanded_index + (parsed_num as usize),
                });
                vec![String::from("."); parsed_num as usize]
            }
            _ => vec![],
        };

        expanded_disk_map.append(&mut expanded_representation);
    }

    (expanded_disk_map, file_locations, free_space_locations)
}

/*
   Algorithm:
   - keep a vecdeque for all numbers of format VecDeque<(source_arr_index, char_value)>
   - pop_back from numbers, pop_front for free space
   - swap popped values,
   - repeat until all free spaces are contiguous
*/
fn re_order_disk(
    expanded_map: Vec<String>,
    mut file_ranges: StorageLocations,
    mut freespace_ranges: StorageLocations,
) -> Vec<String> {
    let mut finished_allotment = false;

    let mut map_clone = expanded_map.clone();
    while !finished_allotment {
        let Some(file_to_order) = file_ranges.pop_front() else {
            finished_allotment = true;
            continue;
        };

        let file_size = file_to_order.end - file_to_order.start;

        for range in freespace_ranges.iter_mut() {
            let freespace_size = range.end - range.start;

            let can_space_fit_num = freespace_size >= file_size;
            let is_file_after_space = file_to_order.start >= range.end;

            if can_space_fit_num && is_file_after_space {
                let new_file_location = Range {
                    start: range.start,
                    end: range.start + file_size,
                };
                range.start = new_file_location.end.clone();
                let file_slice = expanded_map.as_slice()[file_to_order.clone()].to_vec();
                let empty_slice = expanded_map.as_slice()[new_file_location.clone()].to_vec();

                map_clone.splice(new_file_location, file_slice);
                map_clone.splice(file_to_order, empty_slice);
                break;
            }
        }
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
