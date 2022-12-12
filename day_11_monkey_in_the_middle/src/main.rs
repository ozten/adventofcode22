use std::{fs::read_to_string, io::Error};

mod monkey;

mod parser;
use parser::parse;

mod simulator;
use simulator::Simulator;

fn main() -> Result<(), Error> {
    let monkeys = parse("src/input.txt");
    let mut simulator = Simulator::new(monkeys);

    let answer = simulator.simulate();

    println!("Part 1 Answer {answer}");
    let solution: isize = read_to_string("part_2_solution")?.parse().unwrap();
    assert_eq!(answer, solution);

    Ok(())
}
