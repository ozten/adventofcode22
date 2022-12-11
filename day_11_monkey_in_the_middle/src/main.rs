use std::{io::Error, fs::read_to_string};

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
    let solution: isize = read_to_string("part_1_solution")?.parse().unwrap();
    assert_eq!(answer, solution);

    Ok(())
}
