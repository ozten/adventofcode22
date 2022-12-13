use std::{fs::read_to_string, io::Error};

use pathfinding::dijkstra;

mod grid;
use grid::{Grid, Space};

mod parser;
use parser::parse;
use point::Point;

mod point;

/*
(x + 1, y + 2),
(x + 1, y - 2),
(x - 1, y + 2),
(x - 1, y - 2),
(x + 2, y + 1),
(x + 2, y - 1),
(x - 2, y + 1),
(x - 2, y - 1),
*/

fn can_move(from: &Space, to: &Space, to_pair: (i32, i32)) -> Option<(i32, i32)> {
    match (from, to) {
        (Space::Start, Space::Element { path }) => Some(to_pair),
        (Space::Element { path }, Space::End) if path.value == 'z' => Some(to_pair),
        (Space::Start, Space::End) => Some(to_pair),
        (from, to) => match from {
            Space::Element { path } => {
                let from_value = path.value as u32;
                match to {
                    Space::Element { path } if from_value + 1 >= path.value as u32 => {
                        println!("I can move from {from_value} to {:?}", path.value as u32);
                        Some(to_pair)
                    }
                    Space::Element { path } => {
                        println!(
                            "Too steep to move from {from_value} to {:?}",
                            path.value as u32
                        );
                        None
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        _ => None,
    }
}

fn get_as_usize(grid: &Grid, x: i32, y: i32) -> Space {
    let idx: usize = grid.width * y as usize + x as usize;
    grid.spaces.get(idx).unwrap().to_owned()
}

fn valid_neighbors(grid: &Grid, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut valid = Vec::new();
    println!("== Valid Neighbors for {x},{y} ==");

    let cur = get_as_usize(grid, x, y);
    // up
    if y - 1 >= 0 {
        let up = get_as_usize(grid, x, y - 1);
        println!("Looking {x},{y} up {:?}", up);
        match can_move(&cur, &up, (x, y - 1)) {
            Some(pair) => valid.push(pair),
            None => println!("Ignoring up"),
        }
    }

    // right
    if x + 1 < grid.width as i32 {
        let right = get_as_usize(grid, x + 1, y);
        println!("Looking {x},{y} right {:?}", right);
        match can_move(&cur, &right, (x + 1, y)) {
            Some(pair) => valid.push(pair),
            None => println!("Ignoring right"),
        }
    }

    // down
    if y + 1 < grid.height as i32 {
        let down = get_as_usize(grid, x, y + 1);
        println!("Looking at {x},{y} down {:?}", down);
        match can_move(&cur, &down, (x, y + 1)) {
            Some(pair) => valid.push(pair),
            None => println!("Ignoring down"),
        }
    }

    // left
    if x - 1 >= 0 {
        let left = get_as_usize(grid, x - 1, y);
        println!("Looking {x},{y} left {:?}", left);
        match can_move(&cur, &left, (x - 1, y)) {
            Some(pair) => valid.push(pair),
            None => println!("Ignoring left"),
        }
    }

    println!("Valid moves: {:?}", valid);
    println!("");
    valid
}

fn main() -> Result<(), Error> {
    // test input 25
    // 352
    let grid = parse("src/input.txt");

    let goal: (i32, i32) = (grid.goal.x as i32, grid.goal.y as i32);
    let result = dijkstra(
        &(grid.start.x as i32, grid.start.y as i32),
        |&(x, y)| valid_neighbors(&grid, x, y).into_iter().map(|p| (p, 1)),
        |&p| p == goal,
    );
    println!("Answer {:?}", result);
    let solution: isize = read_to_string("part_1_solution")?.parse().unwrap();
    // assert_eq!(result.expect("No path found").1, solution);

    let mut possible_starts: Vec<Option<(i32, i32)>> = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            println!("{x} {y}");
            possible_starts.push(match grid.spaces.get(grid.width * y + x).unwrap() {
                Space::Start => {
                    println!("Detected Start");
                    Some((x as i32, y as i32))
                },
                Space::Element { path } if path.value == 'a' => {
                    println!("Found an a char at {:?} {x}, {y}", path.value);
                Some((x as i32, y as i32))
            },
                _ => None,
            });
        }
    }

    let mut lowest = i32::MAX;

    possible_starts.iter().filter(|pos| pos.is_some()).for_each(|pos| {
        println!("{:?}", pos);
        let result = dijkstra(
            &pos.unwrap(),
            |&(x, y)| valid_neighbors(&grid, x, y).into_iter().map(|p| (p, 1)),
            |&p| p == goal,
        );
        match &result {
            Some(details) => {
                println!("{:?}", details.1);
                if lowest > details.1 {
                    lowest = details.1;
                }
            }
            None => {}
        }
        println!("Answer {:?}", &result);
    });

    println!("Final answer {lowest}");
    let solution: isize = read_to_string("part_2_solution")?.parse().unwrap();
    assert_eq!(lowest, solution as i32);

    Ok(())
}
