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

fn solve(stacks: &mut Vec<VecDeque<String>>, instructions: &Vec<Instructions>) -> String {
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

    let mut tops: Vec<String> = Vec::new();
    for stack in stacks {
        tops.push(stack.pop_front().unwrap());
    }

    tops.join("")
}

#[test]
fn test_overlap() -> Result<(), Box<dyn Error>> {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut instructions: Vec<Instructions> = Vec::new();
    parse(
        read_to_string("src/test_input.txt")?,
        &mut stacks,
        &mut instructions,
    );

    assert_eq!(solve(&mut stacks, &instructions), "CMZ");
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

    let answer = solve(&mut stacks, &instructions);
    println!("{:?}", answer);

    assert_eq!(answer, "BZLVHBWQF");

    Ok(())
}
