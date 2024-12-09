use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = (u32, u32);
type GridMap = Vec<Vec<char>>;

type CoordSet = HashSet<Coord>;

#[derive(Debug, Clone, Copy)]
enum MovementDirection {
    North,
    East,
    South,
    West,
}

/*
   Tracking Struct for guard (PC)
   - current position
   - direction they will be moving in
   - history of nodes it has occupied
   - an ordered subset of the most-recent historical events with derived and meta data
   - a collection of sets, each representing trails made in the given direction
   - the number of possible infinite loops
*/
struct RouteTracker {
    current_position: Coord,
    movement_direction: MovementDirection,
    visited_coords: CoordSet,
    // path_history keeps track of (up to) the last 5 blockers encountered
    path_history: VecDeque<HistoryNode>,
    num_potenatial_loops: i32,
    trail_log: TrailLogger,
}

/*
    Keeps track of pathing history
    Should track:
    - coordinate of this blocker
    - actual coordinate guard turned on
    - what direction was moved after hitting the blocker
    - length of path
*/
struct HistoryNode {
    blocker_xy: Option<Coord>,
    pivot_xy: Coord,
    outgoing_direction: MovementDirection,
    leg_length: i32,
}

/*
    key: relevant [row | column] marker depending on direction
    value: HashSet of [column | row](opposite whatever the key is) values representing starting nodes for trails
*/
type TrailLog = HashMap<i32, HashSet<i32>>;
// type TrailLog = (i32, (i32, i32));

struct TrailLogger {
    north_trails: TrailLog,
    east_trails: TrailLog,
    south_trails: TrailLog,
    west_trails: TrailLog,
}

impl RouteTracker {
    pub fn new(starting_pos: Coord) -> Self {
        Self {
            current_position: starting_pos,
            movement_direction: MovementDirection::North,
            visited_coords: HashSet::<Coord>::from([starting_pos]),
            path_history: VecDeque::<HistoryNode>::from([HistoryNode {
                blocker_xy: None,
                pivot_xy: starting_pos,
                outgoing_direction: MovementDirection::North,
                leg_length: 0,
            }]),
            trail_log: TrailLogger {
                north_trails: HashMap::new(),
                east_trails: HashMap::new(),
                south_trails: HashMap::new(),
                west_trails: HashMap::new(),
            },
            num_potenatial_loops: 0,
        }
    }

    // retrieves next coordinate on the route from a grid described by input
    fn get_next_coord(self: &mut Self, grid: &GridMap) -> (Coord, bool) {
        let num_rows: u32 = grid.len() as u32;
        let num_columns: u32 = grid[0].len() as u32;

        let mut next_x = self.current_position.0;
        let mut next_y = self.current_position.1;

        let (out_of_bounds, _) = match self.movement_direction {
            MovementDirection::North => {
                let is_next_in_bounds = self.current_position.1 != 0;
                if is_next_in_bounds {
                    let potential_next_y = self.current_position.1 - 1;
                    next_y = potential_next_y;

                    match self.trail_log.east_trails.get(&(next_y as i32)) {
                        Some(_) => {
                            // for a sniffed trail where z1 denotes the start of the trail and zx denotes the current position:
                            // there must be no blockers on the path from zx -> z1 to form a valid loop

                            // need to figure out a way to check if there is a trail on path ahead
                        }
                        None => {}
                    }
                }

                (!is_next_in_bounds, 0)
            }
            MovementDirection::East => {
                let potential_next_x = self.current_position.0 + 1;
                let is_next_in_bounds = potential_next_x <= num_columns;
                if is_next_in_bounds {
                    next_x = potential_next_x;

                    match self.trail_log.south_trails.get(&(next_x as i32)) {
                        Some(_) => {}
                        None => {}
                    }
                }

                (!is_next_in_bounds, 0)
            }
            MovementDirection::South => {
                let potential_next_y = self.current_position.1 + 1;
                let is_next_in_bounds = potential_next_y <= num_rows;
                if is_next_in_bounds {
                    next_y = potential_next_y;
                }

                match self.trail_log.west_trails.get(&(next_y as i32)) {
                    Some(_) => {}
                    None => {}
                }

                (!is_next_in_bounds, 0)
            }
            MovementDirection::West => {
                let is_next_in_bounds = self.current_position.0 != 0;
                if is_next_in_bounds {
                    let potential_next_x = self.current_position.0 - 1;
                    next_x = potential_next_x;
                }

                match self.trail_log.north_trails.get(&(next_x as i32)) {
                    Some(_) => {}
                    None => {}
                }

                (!is_next_in_bounds, 0)
            }
        };

        if out_of_bounds {
            // there can possibly be a loop using the path that ends up out of bounds
            self.track_history_event(self.current_position, grid);
        }

        // advanced path loop
        /*
           NEEDS:
           - be able to sniff a trail
               this includes making sure the trail is coming from UPSTREAM THE RELEVANT DIRECTION
           - check the range of values

           where z represents the relative grid position in respect to any direction of movement where 0 is origin and 1 is destination
           for a sniffed trail where z1 denotes the start of the trail and zx denotes the current position:
               there must be no blockers on the path from zx -> z1 to form a valid loop
        */

        ((next_x, next_y), out_of_bounds)
    }

    // handles updating internal data for movement, should also track data for infinite loop checking
    fn advance(self: &mut Self, is_next_blocked: bool, next_pos: Coord, grid: &GridMap) {
        if is_next_blocked {
            match self.movement_direction {
                MovementDirection::North => self.movement_direction = MovementDirection::East,
                MovementDirection::East => self.movement_direction = MovementDirection::South,
                MovementDirection::South => self.movement_direction = MovementDirection::West,
                MovementDirection::West => self.movement_direction = MovementDirection::North,
            }

            // get abs value of leg traveled to get to this position
            self.track_history_event(next_pos, grid);
        } else {
            self.current_position = next_pos;
            self.visited_coords.insert(self.current_position.clone());
        }
    }

    fn track_history_event(self: &mut Self, next_pos: Coord, grid: &GridMap) {
        // create derived data
        let mut distance_traveled: u32 = 0;
        if let Some(last_event) = self.path_history.back() {
            let (last_x, last_y) = last_event.pivot_xy;
            let (current_x, current_y) = self.current_position;

            // need to add 1 as both pivot positions are inclusive
            distance_traveled = match last_event.outgoing_direction {
                // last_y - current_y + 1= yd
                MovementDirection::North => {
                    let delta = last_y.abs_diff(current_y) + 1;
                    match self.trail_log.north_trails.get_mut(&(last_x as i32)) {
                        Some(possible_rows) => {
                            possible_rows.insert(last_y as i32);
                        }
                        None => {
                            self.trail_log
                                .north_trails
                                .insert(last_x as i32, HashSet::from([last_y as i32]));
                        }
                    };

                    delta
                }
                // current_x - last_x + 1= xd
                MovementDirection::East => {
                    let delta = current_x.abs_diff(last_x) + 1;
                    match self.trail_log.east_trails.get_mut(&(last_x as i32)) {
                        Some(possible_rows) => {
                            possible_rows.insert(last_y as i32);
                        }
                        None => {
                            self.trail_log
                                .east_trails
                                .insert(last_y as i32, HashSet::from([last_x as i32]));
                        }
                    };

                    delta
                }
                // current_y - last_y + 1= yd
                MovementDirection::South => {
                    let delta = current_y.abs_diff(last_y) + 1;
                    match self.trail_log.south_trails.get_mut(&(last_x as i32)) {
                        Some(possible_rows) => {
                            possible_rows.insert(last_y as i32);
                        }
                        None => {
                            self.trail_log
                                .south_trails
                                .insert(last_x as i32, HashSet::from([last_y as i32]));
                        }
                    };

                    delta
                }
                // last_x - current_x + 1= xd
                MovementDirection::West => {
                    let delta = last_x.abs_diff(current_x) + 1;
                    match self.trail_log.west_trails.get_mut(&(last_x as i32)) {
                        Some(possible_rows) => {
                            possible_rows.insert(last_y as i32);
                        }
                        None => {
                            self.trail_log
                                .west_trails
                                .insert(last_y as i32, HashSet::from([last_x as i32]));
                        }
                    };

                    delta
                }
            }
        };

        // create and add the new event
        let new_event = HistoryNode {
            blocker_xy: Some(next_pos),
            pivot_xy: self.current_position.clone(),
            outgoing_direction: self.movement_direction.clone(),
            leg_length: distance_traveled as i32,
        };
        self.path_history.push_back(new_event);

        // make sure only the MOST RECENT 5 nodes are kept
        while self.path_history.len() > 5 {
            self.path_history.pop_front();
        }

        if self.can_infinite_loop(grid) {
            self.num_potenatial_loops += 1;
        }
    }

    fn can_infinite_loop(self: &Self, grid: &GridMap) -> bool {
        /*
           A rectangular loop has the following definition:
           - leg 1 and leg 3 are equivalent
           - leg 2 and leg 4 are equivalent

           ^ the above is the definition of a rectangle (which encompases squares) defining a closed shape
             of four right angles (the right turns); a loop


           SCENARIO 1:
               if (1.leg_length >= 3. leg_length) && (4.leg_length > 2.leg_length) --> valid potential loop
           Scenario 2:
               if (1.leg_length < 3. leg_length) && ((4.leg_length >= 2.leg_length)) --> need to check check for blockers between potential(x, y) -> 0.(x, y)

            There is a final scenario:
                For each unit being moved over, if a pivot were to happen on that coordiante and there is a recorded
                upstream pivot point, then it is possible to enter an infinite loop.
                (This happens by putting the guard back onto the path it took to reach this point => leading to the loop)
        */
        let mut is_loop_possible = false;

        // basic cases
        if self.path_history.len() == 5 {
            let event_0 = &self.path_history[0];
            let event_1 = &self.path_history[1];
            let event_2 = &self.path_history[2];
            let event_3 = &self.path_history[3];
            let event_4 = &self.path_history[4];

            // because there are no naturally ocurring loops,
            // if (2.leg == 4.leg):
            //       we can deduce the existence of a blocker between 4.pivot point and 1.pivot point
            //      (an un-interupted path must exist between 4.pivot and 1.pivot to complete a full loop)
            let is_impossible_by_deduction = event_2.leg_length == event_4.leg_length;
            // since 4.length is the final leg to be calculated in the sequence, it cannot be smaller
            // than its parallel length (2.leg) which has already been calculated
            let is_impossible_by_calculation = event_2.leg_length > event_4.leg_length;

            if !is_impossible_by_deduction && !is_impossible_by_calculation {
                let new_pivot_point = match event_3.outgoing_direction {
                    // new pivot_point = (3.x, 1.y)
                    MovementDirection::North | MovementDirection::South => {
                        (event_3.pivot_xy.0, event_1.pivot_xy.1)
                    }
                    // new pivot_point = (3.x, 1.y)
                    // MovementDirection::South => (event_3.pivot_xy.0, event_1.pivot_xy.1),
                    // new pivot_point = (1.x, 3.y)
                    MovementDirection::East | MovementDirection::West => {
                        (event_1.pivot_xy.0, event_3.pivot_xy.1)
                    } // new pivot_point = (1.x, 3.y)
                      // MovementDirection::West => (event_1.pivot_xy.0, event_3.pivot_xy.1),
                };
                let new_direction: MovementDirection = event_4.outgoing_direction;

                // in this instance we simply know to add a blocker via deduction
                if event_1.leg_length >= event_3.leg_length {
                    // new blocker_coord = pivot_point + match direction { add/sub x/y per case}
                    is_loop_possible = true;
                }
                // in this case, there is a chance a blocker exists in the path from
                // potential_pivot_point(x, y) -> 0.pivot(x, y)
                if event_1.leg_length < event_3.leg_length {
                    let (new_x, new_y) = new_pivot_point;
                    let (x_0, y_0) = event_0.pivot_xy;

                    let subset = match new_direction {
                        MovementDirection::North => {
                            let mut subset: Vec<char> = Vec::new();
                            for row in new_y..y_0 {
                                subset.push(grid[row as usize][new_x as usize].clone())
                            }
                            subset
                        }
                        // new pivot_point = (3.x, 1.y)
                        MovementDirection::South => {
                            let mut subset: Vec<char> = Vec::new();
                            for row in y_0..new_y {
                                subset.push(grid[row as usize][new_x as usize].clone())
                            }
                            subset
                        }
                        MovementDirection::East => {
                            let subset = grid[new_pivot_point.1 as usize]
                                [(new_x as usize)..(x_0 as usize)]
                                .to_vec();
                            subset
                        }
                        MovementDirection::West => {
                            let subset = grid[new_pivot_point.1 as usize]
                                [(x_0 as usize)..(new_x as usize)]
                                .to_vec();
                            subset
                        }
                    };

                    is_loop_possible = !subset.contains(&'#');
                }
                println!(
                    "is loop possible: {} | at: {:?}",
                    is_loop_possible, new_pivot_point
                )
            }
        }

        // advanced path loop
        /*
           NEEDS:
           - be able to sniff a trail
               this includes making sure the trail is coming from UPSTREAM THE RELEVANT DIRECTION
           - check the range of values

           where z represents the relative grid position in respect to any direction of movement where 0 is origin and 1 is destination
           for a sniffed trail where z1 denotes the start of the trail and zx denotes the current position:
               there must be no blockers on the path from zx -> z1 to form a valid loop
        */

        is_loop_possible
    }
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

    let route_tracker = traverse_grid_v2(grid, starting_pos, hashtags);

    println!(
        "sum spots visited: {} | possible loops: {}",
        route_tracker.visited_coords.len(),
        route_tracker.num_potenatial_loops
    )

    // NEW: need to figure out the count of configurations where you can add 1 hashtag to cause an infinite loop

    /*
       How to solve:
       Will need to keep a rolling track of the last 4 blockers visited.

    */
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
            let current_position_xy: Coord = ((column_index as u32), (row_index as u32));
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

/* With tracking! */
fn traverse_grid_v2(
    grid: GridMap,
    starting_pos: Coord,
    blocker_locations: CoordSet,
) -> RouteTracker {
    let mut route_tracker: RouteTracker = RouteTracker::new(starting_pos);

    let mut in_bounds = true;
    while in_bounds {
        let (next_coord, next_out_of_bounds): (Coord, bool) = route_tracker.get_next_coord(&grid);
        if next_out_of_bounds {
            in_bounds = false;
            continue;
        } else {
            let is_next_blocked = blocker_locations.contains(&next_coord);
            route_tracker.advance(is_next_blocked, next_coord, &grid);
        }
    }

    route_tracker
}
