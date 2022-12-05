use std::collections::VecDeque;
use std::fs::read_to_string;

use std::error::Error;

mod parser;
use parser::parse;

#[derive(Debug, PartialEq)]
pub struct Instructions {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

fn crate_mover_9000(stacks: &mut Vec<VecDeque<String>>, instructions: &Vec<Instructions>) {
    for instr in instructions {
        let mut idx = 0;
        loop {
            if idx < instr.amount {
                let tmp = stacks.get_mut(instr.from - 1).unwrap().pop_front().unwrap();
                stacks.get_mut(instr.to - 1).unwrap().push_front(tmp);
                idx += 1;
            } else {
                break;
            }
        }
    }
}

fn crate_mover_9001(stacks: &mut Vec<VecDeque<String>>, instructions: &Vec<Instructions>) {
    for instr in instructions {
        let mut idx = 0;
        let mut column: VecDeque<String> = VecDeque::new();

        loop {
            if idx < instr.amount {
                column.push_front(stacks.get_mut(instr.from - 1).unwrap().pop_front().unwrap());
                idx += 1;
            } else {
                break;
            }
        }
        idx = 0;
        loop {
            if idx < instr.amount {
                let tmp = column.pop_front().unwrap();
                stacks.get_mut(instr.to - 1).unwrap().push_front(tmp);
                idx += 1;
            } else {
                break;
            }
        }
    }
}

fn solve(
    stacks: &mut Vec<VecDeque<String>>,
    instructions: &Vec<Instructions>,
    is_9001: bool,
) -> String {
    if is_9001 {
        crate_mover_9001(stacks, instructions);
    } else {
        crate_mover_9000(stacks, instructions);
    }

    let mut tops: Vec<String> = Vec::new();
    for stack in stacks {
        tops.push(stack.pop_front().unwrap());
    }

    tops.join("")
}

#[test]
fn test_solve_9000() -> Result<(), Box<dyn Error>> {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut instructions: Vec<Instructions> = Vec::new();
    parse(
        read_to_string("src/test_input.txt")?,
        &mut stacks,
        &mut instructions,
    );

    assert_eq!(solve(&mut stacks, &instructions, false), "CMZ");
    Ok(())
}

#[test]
fn test_solve_9001() -> Result<(), Box<dyn Error>> {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut instructions: Vec<Instructions> = Vec::new();
    parse(
        read_to_string("src/test_input.txt")?,
        &mut stacks,
        &mut instructions,
    );

    assert_eq!(solve(&mut stacks, &instructions, true), "MCD");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut instructions: Vec<Instructions> = Vec::new();
    parse(
        read_to_string("src/input.txt")?,
        &mut stacks,
        &mut instructions,
    );

    let answer = solve(&mut stacks, &instructions, true);
    println!("{:?}", answer);

    // Part 1 assert_eq!(answer, "BZLVHBWQF");
    // Part 2
    assert_eq!(answer, "TDGJQTZSL");
    Ok(())
}
