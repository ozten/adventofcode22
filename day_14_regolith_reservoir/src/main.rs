mod grid;
use grid::min_max;

mod parser;
use parser::parse;

mod simulation;
use crate::simulation::simulate;

use crate::{grid::Grid, parser::Coordinate};

fn main() {
    let test_mode = env!("TEST_MODE") == "true";

    let sand_coord = Coordinate { x: 500, y: 0 };

    let lines = parse(if test_mode {
        "src/test_input.txt"
    } else {
        "src/input.txt"
    });

    let ((min_x, max_x), (min_y, max_y)) = min_max(&lines, &sand_coord);

    let mut grid = Grid::new(min_x, max_x, min_y, max_y);
    grid.record_rock(&lines);

    simulate(&mut grid);    

    println!("{:?}", grid);

    println!("{min_x},{min_y} {max_x},{max_y}");
}
