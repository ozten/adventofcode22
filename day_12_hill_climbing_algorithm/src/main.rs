use std::{fs::read_to_string, io::Error};

use pathfinding::dijkstra;

mod grid;
use grid::{Grid, Space};

mod parser;
use parser::parse;
use point::Point;

mod point;

fn main() -> Result<(), Error> {
    let test_mode = env!("TEST_MODE") == "true";

    let grid = parse(if test_mode {
        "src/test_input.txt"
    } else {
        "src/input.txt"
    });

    let result: Option<(Vec<Point>, usize)> = dijkstra(
        &grid.start,
        |p| grid.valid_neighbors( &p),
        |p| *p == grid.goal,
    );

    let solution: usize = read_to_string("part_1_solution")?.parse().unwrap();
    let answer = result.expect("No path found").1;

    println!("Answer {answer}");

    if test_mode {
        assert_eq!(answer, 31);
    } else {
        assert_eq!(answer, solution);
    }

    // Part 2
    let mut possible_starts: Vec<Option<Point>> = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            possible_starts.push(match grid.spaces.get(grid.width * y + x).unwrap() {
                Space::Start => Some(Point { x, y }),
                Space::Element { path } if path.value == 'a' => Some(Point { x, y }),
                _ => None,
            });
        }
    }

    let mut lowest = usize::MAX;

    possible_starts
        .iter()
        .filter(|pos| pos.is_some())
        .for_each(|pos| {
            let result = dijkstra(
                pos.as_ref().unwrap(),
                |&p| grid.valid_neighbors( &p),
                |&p| p == grid.goal,
            );
            match &result {
                Some(details) => {
                    if lowest > details.1 {
                        lowest = details.1;
                    }
                }
                None => {}
            }
        });
    println!("Final answer {lowest}");
    let solution: usize = read_to_string("part_2_solution")?.parse().unwrap();
    if test_mode {
        assert_eq!(lowest, 29);
    } else {
        assert_eq!(lowest, solution);
    }

    Ok(())
}
