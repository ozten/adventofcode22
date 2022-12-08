use std::fs::read_to_string;

use std::io::Error;

fn parse(input: String) -> (usize, usize, Vec<u32>) {
    let mut height = 0;
    let mut width = 0;
    let mut trees: Vec<u32> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        width = line.len();
        height += 1;
        for c in line.chars() {
            let val = c.to_digit(10).unwrap();
            trees.push(val);
        }
    }
    (width, height, trees)
}

#[test]
fn test_parse() -> Result<(), Error> {
    let (actual_width, actual_height, actual_input) = parse(read_to_string("src/test_input.txt")?);

    assert_eq!(actual_width, 5);
    assert_eq!(actual_height, 5);
    assert_eq!(*actual_input.get(0).unwrap(), 3);
    assert_eq!(*actual_input.get(24).unwrap(), 0);

    Ok(())
}

#[test]
fn test_parse_big() -> Result<(), Error> {
    let (actual_width, actual_height, actual_input) = parse(read_to_string("src/input.txt")?);

    assert_eq!(actual_width, 99);
    assert_eq!(actual_height, 99);
    assert_eq!(*actual_input.get(0).unwrap(), 1);
    assert_eq!(*actual_input.get(24).unwrap(), 5);

    Ok(())
}

fn solve(width: usize, height: usize, trees: Vec<u32>) -> usize {
    let mut visible_count = 0;
    for x in 0..width {
        for y in 0..height {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                visible_count += 1;
            } else {
                if tree_is_visible_row(x, y, width, height, &trees)
                    || tree_is_visible_column(x, y, height, &trees)
                {
                    visible_count += 1;
                }
            }
        }
    }
    visible_count
}

fn index_at(x: usize, y: usize, height: usize) -> usize {
    y * height + x
}

fn tree_is_visible_row(x: usize, y: usize, width: usize, height: usize, trees: &Vec<u32>) -> bool {
    let idx = index_at(x, y, height);
    let tree_height = trees.get(idx).unwrap();

    let mut left_clear = true;
    for col in 0..x {
        let col_tree_height = trees.get(index_at(col, y, height)).unwrap();
        if col_tree_height >= tree_height {
            left_clear = false;
            break;
        }
    }

    let mut right_clear = true;
    for col in x + 1..width {
        let col_tree_height = trees.get(index_at(col, y, height)).unwrap();
        if col_tree_height >= tree_height {
            right_clear = false;
            break;
        }
    }
    left_clear || right_clear
}

fn tree_is_visible_column(x: usize, y: usize, height: usize, trees: &Vec<u32>) -> bool {
    north_is_clear(x, y, height, trees) || south_is_clear(x, y, height, trees)
}

#[test]
fn test_tree_is_visible_column_north() {
    let actual = tree_is_visible_column(
        2,
        2,
        5,
        &vec![
            1, 4, 1, 4, 1, 1, 4, 2, 4, 1, 1, 4, 3, 4, 1, 1, 4, 4, 4, 1, 1, 4, 1, 4, 1,
        ],
    );
    assert_eq!(actual, true);
}

#[test]
fn test_tree_is_not_visible_column_north() {
    let actual = tree_is_visible_column(
        2,
        2,
        5,
        &vec![
            1, 4, 1, 4, 1, 1, 4, 4, 4, 1, 1, 4, 3, 4, 1, 1, 4, 4, 4, 1, 1, 4, 1, 4, 1,
        ],
    );
    assert_eq!(actual, false);
}

fn north_is_clear(x: usize, y: usize, height: usize, trees: &[u32]) -> bool {
    let idx = index_at(x, y, height);
    let tree_height = trees.get(idx).unwrap();

    let mut north_clear = true;
    for row in 0..y {
        let row_tree_height = trees.get(index_at(x, row, height)).unwrap();
        if row_tree_height >= tree_height {
            north_clear = false;
            break;
        }
    }
    north_clear
}

fn south_is_clear(x: usize, y: usize, height: usize, trees: &[u32]) -> bool {
    let idx = index_at(x, y, height);
    let tree_height = trees.get(idx).unwrap();

    let mut south_clear = true;
    for row in y + 1..height {
        let row_tree_height = trees.get(index_at(x, row, height)).unwrap();
        if row_tree_height >= tree_height {
            south_clear = false;
            break;
        }
    }
    south_clear
}

#[test]
fn test_solve() -> Result<(), Error> {
    let (width, height, trees) = parse(read_to_string("src/test_input.txt")?);
    let actual = solve(width, height, trees);
    assert_eq!(actual, 21);
    Ok(())
}

fn main() -> Result<(), Error> {
    let (width, height, trees) = parse(read_to_string("src/input.txt")?);
    let answer = solve(width, height, trees);
    println!("Answer Part#1 {:?}", answer);
    let solution: usize = read_to_string("part_1_solution")?
        .parse().unwrap();

    assert_eq!(answer, solution);

    Ok(())
}
