use std::cmp::max;

use crate::matrix::index_at;

pub fn best_scenic_score(width: usize, height: usize, trees: &Vec<u32>) -> usize {
    let mut best_score_seen = 0;
    for x in 0..width {
        for y in 0..height {
            let scenic_score = calculate_scenic_score(x, y, width, height, trees);
            if scenic_score > best_score_seen {
                best_score_seen = scenic_score;
            }
        }
    }
    best_score_seen
}

fn calculate_scenic_score(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    trees: &Vec<u32>,
) -> usize {
    max(1, north_score(x, y, height, trees))
        * max(1, east_score(x, y, width, height, trees))
        * max(1, south_score(x, y, height, trees))
        * max(1, west_score(x, y, height, trees))
}

fn north_score(x: usize, y: usize, height: usize, trees: &[u32]) -> usize {
    if y == 0 {
        return 0;
    }
    let idx = index_at(x, y, height);
    let tree_height = trees.get(idx).unwrap();

    let mut north_count = 0;

    let mut row = y - 1;
    loop {
        let row_tree_height = trees.get(index_at(x, row, height)).unwrap();
        north_count += 1;
        if row_tree_height >= tree_height {
            break;
        }
        if row == 0 {
            break;
        }
        row -= 1;
    }
    north_count
}

fn east_score(x: usize, y: usize, width: usize, height: usize, trees: &Vec<u32>) -> usize {
    if x == width - 1 {
        return 0;
    }
    let idx = index_at(x, y, height);
    let tree_height = trees.get(idx).unwrap();

    let mut east_count = 0;

    for col in x + 1..width {
        let col_tree_height = trees.get(index_at(col, y, height)).unwrap();
        east_count += 1;
        if col_tree_height >= tree_height {
            break;
        }
    }
    east_count
}

fn south_score(x: usize, y: usize, height: usize, trees: &[u32]) -> usize {
    if y == height - 1 {
        return 0;
    }

    let idx = index_at(x, y, height);
    let tree_height = trees.get(idx).unwrap();

    let mut south_count = 0;
    for row in y + 1..height {
        let row_tree_height = trees.get(index_at(x, row, height)).unwrap();
        south_count += 1;
        if row_tree_height >= tree_height {
            break;
        }
    }
    south_count
}

fn west_score(x: usize, y: usize, height: usize, trees: &Vec<u32>) -> usize {
    if x == 0 {
        return 0;
    }

    let idx = index_at(x, y, height);
    let tree_height = trees.get(idx).unwrap();

    let mut west_count = 0;
    let mut col = x - 1;
    loop {
        let col_tree_height = trees.get(index_at(col, y, height)).unwrap();
        west_count += 1;
        if col_tree_height >= tree_height {
            break;
        }

        if col == 0 {
            break;
        }
        col -= 1;
    }
    west_count
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use std::fs::read_to_string;
    use std::io::Error;

    use crate::scenic_score::best_scenic_score;

    #[test]
    fn test_trees_visible_count() -> Result<(), Error> {
        let (width, height, trees) = parse(read_to_string("src/test_input.txt")?);
        let actual = best_scenic_score(width, height, &trees);
        assert_eq!(actual, 16);
        Ok(())
    }

    #[test]
    fn test_max_5x5_score() -> Result<(), Error> {
        let (width, height, trees) = parse(read_to_string("src/test_input_max.txt")?);
        let actual = best_scenic_score(width, height, &trees);
        assert_eq!(actual, 12);
        Ok(())
    }
}
