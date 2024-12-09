use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (usize, usize);
type GridMap = Vec<Vec<char>>;

type CoordSet = HashSet<Coord>;

#[derive(Debug)]
enum MovementDirection {
    North,
    East,
    South,
    West,
}

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

    // NOTE: coords ARE 0 indexed to work natively with the data struct
    // NOTE: Grid is stored as rows -> columns, so (x, y) = grid[y][x]
    // NOTE: the value of a row increases as you go down, the value of a column increases as you go right
    let (grid, starting_pos, hashtags) = parse_input_to_grid(input_file);

    let starting_pos = match starting_pos {
        Some(coord) => coord,
        None => {
            println!("No starting position found!");
            return;
        }
    };

    let visited_coords = traverse_grid(grid, starting_pos, hashtags);

    println!("sum spots visited: {}", visited_coords.len())
}

fn parse_input_to_grid(file: File) -> (GridMap, Option<Coord>, CoordSet) {
    let reader: BufReader<File> = BufReader::new(file);
    let lines = reader.lines().flatten();
    let mut grid_2d: GridMap = Vec::new();

    let mut starting_position: Option<Coord> = None;
    let mut hash_locations: CoordSet = HashSet::new();

    for (row_index, line) in lines.enumerate() {
        let chars: std::str::Chars<'_> = line.chars();
        grid_2d.push(chars.clone().collect::<Vec<char>>());

        for (column_index, char) in chars.enumerate() {
            let current_position_xy: Coord = (column_index, row_index);
            match char {
                '#' => {
                    hash_locations.insert(current_position_xy);
                }
                '^' => {
                    starting_position = Some(current_position_xy);
                }
                _ => {}
            }
        }
    }

    (grid_2d, starting_position, hash_locations)
}

fn traverse_grid(grid: GridMap, starting_pos: Coord, blocker_locations: CoordSet) -> CoordSet {
    let num_rows: usize = grid.len();
    let num_columns: usize = grid[0].len();
    let (x_start, y_start) = starting_pos;

    let mut visited_coords: CoordSet = HashSet::from([starting_pos]);
    let mut movement_direction: MovementDirection = MovementDirection::North;
    let mut in_bounds: bool = true;

    let mut current_position: Coord = (x_start, y_start);

    /* Algorithm:
       define a set of 'visited coordinates' that starts with starting position
       - while: check to see if next movement position is in bounds
           - true?
             check to see if its a blocker
               - true? change direction, continue to next iteration
               - false?
                   set current_position = next position
                   add current_position to visited_coords
           - false?
               break loop and begin to sum
    */
    while in_bounds {
        let (next_coord, next_out_of_bounds): (Coord, bool) =
            get_next_coord(current_position, &movement_direction, num_rows, num_columns);
        if next_out_of_bounds {
            in_bounds = false;
            continue;
        }

        let is_next_blocked = blocker_locations.contains(&next_coord);
        if is_next_blocked {
            match movement_direction {
                MovementDirection::North => movement_direction = MovementDirection::East,
                MovementDirection::East => movement_direction = MovementDirection::South,
                MovementDirection::South => movement_direction = MovementDirection::West,
                MovementDirection::West => movement_direction = MovementDirection::North,
            }
            continue;
        }

        current_position = next_coord;
        visited_coords.insert(current_position.clone());
    }

    visited_coords
}

fn get_next_coord(
    current_position: Coord,
    movement_direction: &MovementDirection,
    num_rows: usize,
    num_columns: usize,
) -> (Coord, bool) {
    let mut next_x = current_position.0;
    let mut next_y = current_position.1;

    let mut out_of_bounds = false;

    match movement_direction {
        MovementDirection::North => {
            let is_next_in_bounds = current_position.1 != 0;
            if is_next_in_bounds {
                let potential_next_y = current_position.1 - 1;
                next_y = potential_next_y as usize
            } else {
                out_of_bounds = true;
            }
        }
        MovementDirection::East => {
            let potential_next_x = current_position.0 + 1;
            let is_next_in_bounds = potential_next_x <= num_columns;
            if is_next_in_bounds {
                next_x = potential_next_x
            } else {
                out_of_bounds = true;
            }
        }
        MovementDirection::South => {
            let potential_next_y = current_position.1 + 1;
            let is_next_in_bounds = potential_next_y <= num_rows;
            if is_next_in_bounds {
                next_y = potential_next_y
            } else {
                out_of_bounds = true;
            }
        }
        MovementDirection::West => {
            let is_next_in_bounds = current_position.0 != 0;
            if is_next_in_bounds {
                let potential_next_x = current_position.0 - 1;
                next_x = potential_next_x as usize
            } else {
                out_of_bounds = true;
            }
        }
    }

    ((next_x, next_y), out_of_bounds)
}
