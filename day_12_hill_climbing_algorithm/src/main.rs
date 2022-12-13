use std::{fs::read_to_string, io::Error};

use pathfinding::dijkstra;

mod grid;
use grid::{Grid, Space};

mod parser;
use parser::parse;
use point::Point;

mod point;

fn get_as_usize(grid: &Grid, point: &Point) -> Space {
    grid.spaces
        .get(grid.width * point.y + point.x)
        .unwrap()
        .to_owned()
}

fn get_as_usize_offset_row(grid: &Grid, point: &Point, y: isize) -> Space {
    let new_y = point.y as isize + y;
    grid.spaces
        .get(grid.width * (new_y as usize) + point.x)
        .unwrap()
        .to_owned()
}

fn get_as_usize_offset_col(grid: &Grid, point: &Point, x: isize) -> Space {
    let new_x = point.x as isize + x;
    grid.spaces
        .get(grid.width * point.y + new_x as usize)
        .unwrap()
        .to_owned()
}

fn can_move(from: &Space, to: &Space, to_pair: &Point) -> Option<Point> {
    match from {
        Space::Start => Some(to_pair.to_owned()),
        // End of the road!
        Space::Element { path } if path.value == 'z' => Some(to_pair.to_owned()),
        Space::Element { path } => {
            let from_value = path.value as u32;
            match to {
                // Approachable
                Space::Element { path } if from_value + 1 >= path.value as u32 => {
                    Some(to_pair.to_owned())
                }
                // Too steep for remaining Space::Element { path }
                _ => None,
            }
        }
        _ => None,
    }
}

fn valid_neighbors(grid: &Grid, point: &Point) -> Vec<(Point, usize)> {
    let mut valid = Vec::new();
    let cur = get_as_usize(grid, point);
    // up
    if point.y >= 1 {
        let up = get_as_usize_offset_row(grid, point, -1);
        match can_move(
            &cur,
            &up,
            &Point {
                x: point.x,
                y: point.y - 1,
            },
        ) {
            Some(pair) => valid.push(pair),
            None => {}
        }
    }

    // right
    if point.x + 1 < grid.width {
        let right = get_as_usize_offset_col(grid, point, 1);
        match can_move(
            &cur,
            &right,
            &Point {
                x: point.x + 1,
                y: point.y,
            },
        ) {
            Some(pair) => valid.push(pair),
            None => {}
        }
    }

    // down
    if point.y + 1 < grid.height {
        let down = get_as_usize_offset_row(grid, point, 1);
        match can_move(
            &cur,
            &down,
            &Point {
                x: point.x,
                y: point.y + 1,
            },
        ) {
            Some(pair) => valid.push(pair),
            None => {}
        }
    }

    // left
    if point.x >= 1 {
        let left = get_as_usize_offset_col(grid, point, -1);
        match can_move(
            &cur,
            &left,
            &Point {
                x: point.x - 1,
                y: point.y,
            },
        ) {
            Some(pair) => valid.push(pair),
            None => {}
        }
    }
    valid.into_iter().map(|p| (p, 1)).collect()
}

fn main() -> Result<(), Error> {
    let test_mode = env!("TEST_MODE") == "true";

    let grid = parse(if test_mode {
        "src/test_input.txt"
    } else {
        "src/input.txt"
    });

    let result: Option<(Vec<Point>, usize)> = dijkstra(
        &grid.start,
        |p| valid_neighbors(&grid, &p),
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
                |&p| valid_neighbors(&grid, &p),
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
