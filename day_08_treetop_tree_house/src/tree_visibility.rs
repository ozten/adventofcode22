use crate::matrix::index_at;
pub fn trees_visible_count(width: usize, height: usize, trees: &Vec<u32>) -> usize {
    let mut visible_count = 0;
    for x in 0..width {
        for y in 0..height {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                visible_count += 1;
            } else {
                if tree_is_visible_row(x, y, width, height, trees)
                    || tree_is_visible_column(x, y, height, trees)
                {
                    visible_count += 1;
                }
            }
        }
    }
    visible_count
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

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use std::fs::read_to_string;
    use std::io::Error;

    use crate::tree_visibility::trees_visible_count;

    #[test]
    fn test_trees_visible_count() -> Result<(), Error> {
        let (width, height, trees) = parse(read_to_string("src/test_input.txt")?);
        let actual = trees_visible_count(width, height, &trees);
        assert_eq!(actual, 21);
        Ok(())
    }
}
