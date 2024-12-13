use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops::Mul;

// (x, y) coordinate
type Coord = (usize, usize);
// representation of entire input
type AreaMap = Vec<Vec<Plot>>;
type Region = HashSet<Plot>;

#[derive(Clone, Debug)]
struct Plot {
    plant_type: char,
    clashing_perimeter: usize,
    location: Coord,
}

impl Hash for Plot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.location.hash(state);
    }
}

impl Eq for Plot {}
impl PartialEq for Plot {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

impl Plot {
    fn new(location: Coord, char: char) -> Self {
        Plot {
            plant_type: char,
            clashing_perimeter: 4,
            location,
        }
    }

    /*
       returns tuple of order (north, east, south, west)
           each value is an Option, a value of some indicates there is a matching plot at the given coordinate
           a value of None = there is a border or non-matching field

    */
    fn check_borders(self: &mut Self, area_map: &AreaMap) -> Vec<Plot> {
        let num_rows = area_map.len();
        let num_cols = area_map[0].len();
        // let mut north_border: Option<Coord> = None;
        // let mut east_border: Option<Coord> = None;
        // let mut south_border: Option<Coord> = None;
        // let mut west_border: Option<Coord> = None;

        let mut related_bordering_plots: Vec<Plot> = Vec::new();

        // check north
        let is_north_border = self.location.1 == 0;
        if !is_north_border {
            let north_plot_coord = (self.location.0, self.location.1 - 1);
            let north_plot = area_map[north_plot_coord.1][north_plot_coord.0].clone();

            if north_plot.plant_type == self.plant_type {
                // north_border = Some(north_plot_coord);
                related_bordering_plots.push(north_plot);
            }
        }
        // check east
        let is_east_border = self.location.0 == num_cols - 1;
        if !is_east_border {
            let east_plot_coord = (self.location.0 + 1, self.location.1);
            let east_plot = area_map[east_plot_coord.1][east_plot_coord.0].clone();

            if east_plot.plant_type == self.plant_type {
                // east_border = Some(east_plot_coord);
                related_bordering_plots.push(east_plot);
            }
        }
        // check south
        let is_south_border = self.location.1 == num_rows - 1;
        if !is_south_border {
            let south_plot_coord = (self.location.0, self.location.1 + 1);
            let south_plot = area_map[south_plot_coord.1][south_plot_coord.0].clone();

            if south_plot.plant_type == self.plant_type {
                // south_border = Some(south_plot_coord);
                related_bordering_plots.push(south_plot);
            }
        }
        // check west
        let is_west_border = self.location.0 == 0;
        if !is_west_border {
            let west_plot_coord = (self.location.0 - 1, self.location.1);
            let west_plot = area_map[west_plot_coord.1][west_plot_coord.0].clone();

            if west_plot.plant_type == self.plant_type {
                // west_border = Some(west_plot_coord);
                related_bordering_plots.push(west_plot);
            }
        }

        // (north_border, east_border, south_border, west_border)
        self.clashing_perimeter -= related_bordering_plots.len();
        related_bordering_plots
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

    let reader = BufReader::new(input_file);
    let lines = reader.lines();

    let mut map: AreaMap = Vec::new();
    for (row, line) in lines.enumerate() {
        match line {
            Ok(line) => {
                parse_farm(row, line, &mut map);
            }
            _ => {}
        }
    }
    let regions = create_regions(&map);
    let mut region_costs: Vec<i64> = vec![];
    for region in regions {
        // need to calculate cost n stuff
        let mut perim_len: i64 = 0;
        let mut area: i64 = 0;
        let mut cost: i64 = 0;
        // println!("region: {:?}", region);
        for plot in region {
            area += 1;
            perim_len += plot.clashing_perimeter as i64;
            cost = area.mul(perim_len);
        }
        // println!("area: {}, perim: {}, cost: {}", area, perim_len, cost);
        region_costs.push(cost);
    }

    let total_cost = region_costs.into_iter().sum::<i64>();
    println!("total cost: {}", total_cost)
}

fn parse_farm(row: usize, input_line: String, area_map: &mut AreaMap) {
    let mut new_row = Vec::new();
    for (col, char) in input_line.chars().enumerate() {
        new_row.push(Plot::new((col, row), char));
    }
    area_map.push(new_row)
}

fn create_regions(plot_list: &AreaMap) -> Vec<Region> {
    let mut regions = Vec::new();

    for (_, row_plots) in plot_list.iter().enumerate() {
        for (_, plot) in row_plots.iter().enumerate() {
            let plot_already_registered = regions
                .iter()
                .any(|region: &HashSet<Plot>| region.contains(plot));

            if !plot_already_registered {
                let mut new_region = HashSet::new();
                /*
                   Start from 0,0

                   create a recursive function to expand from a single plot to grab all connected plots, returning a region

                */
                create_region_from_plot(plot_list, &mut plot.clone(), &mut new_region);
                regions.push(new_region)
            }
        }
    }

    regions
}

fn create_region_from_plot(area_map: &AreaMap, plot: &mut Plot, region: &mut Region) {
    let mut related_bordering_plots: Vec<Plot> = plot.check_borders(area_map);
    region.insert(plot.clone());

    for border_plot in related_bordering_plots.iter_mut() {
        if !region.contains(&border_plot) {
            create_region_from_plot(area_map, border_plot, region);
        }
    }
}
