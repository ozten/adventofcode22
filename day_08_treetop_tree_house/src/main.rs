use std::fs::read_to_string;
use std::io::Error;

mod matrix;

mod parser;

use parser::parse;

mod tree_visibility;
use tree_visibility::trees_visible_count;

mod scenic_score;
use scenic_score::best_scenic_score;

fn main() -> Result<(), Error> {
    let (width, height, trees) = parse(read_to_string("src/input.txt")?);
    let answer = trees_visible_count(width, height, &trees);
    println!("Answer Part#1 {:?}", answer);
    let solution: usize = read_to_string("part_1_solution")?.parse().unwrap();

    assert_eq!(answer, solution);

    let answer = best_scenic_score(width, height, &trees);
    println!("Answer Part#2 {:?}", answer);
    let solution: usize = read_to_string("part_2_solution")?.parse().unwrap();

    assert_eq!(answer, solution);

    Ok(())
}
