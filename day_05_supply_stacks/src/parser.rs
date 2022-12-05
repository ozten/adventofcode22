use std::collections::VecDeque;
use std::error::Error;
use std::fs::read_to_string;

use crate::Instructions;

fn parse_container_line<'a>(line: String, stacks: &mut Vec<VecDeque<String>>) {
    let mut idx = 0;

    if stacks.len() == 0 {
        initialize_stacks(&line, stacks);
    }

    loop {
        if idx >= line.len() {
            break;
        }

        match line.get(idx + 1..idx + 2) {
            // Stack column can have empty spaces, don't record as a container
            Some(label) if label.eq(" ") => (),
            Some(label) => match stacks.get_mut(idx / 4) {
                Some(stack) => stack.push_back(label.to_owned()),
                _ => (),
            },
            None => (),
        }
        idx += 4;
    }
}

fn initialize_stacks(line: &String, stacks: &mut Vec<VecDeque<String>>) {
    let mut idx = 0;
    loop {
        if idx >= line.len() {
            break;
        }
        stacks.push(VecDeque::new());
        idx += 4;
    }
}

fn parse_instruction_line(line: &str, instructions: &mut Vec<Instructions>) {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let amount = parts.get(1);
    let from = parts.get(3);
    let to = parts.get(5);

    match (amount, from, to) {
        (Some(amount), Some(from), Some(to)) => instructions.push(Instructions {
            amount: amount.parse().unwrap(),
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        }),
        _ => panic!("Malformed instruction line {line}"),
    }
}

pub fn parse<'a>(
    input: String,
    stacks: &mut Vec<VecDeque<String>>,
    instructions: &mut Vec<Instructions>,
) {
    for line in input.split("\n") {
        match line.get(0..4) {
            Some(part) if part.starts_with("[") || part.eq("    ") => {
                parse_container_line(line.to_owned(), stacks)
            }
            Some(" 1  ") => (),
            Some("move") => parse_instruction_line(line, instructions),
            Some(part) if part.is_empty() => (),
            Some(part) => println!("Ignoring unknown line _{part}_"),
            None => (),
        }
    }
}

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let mut stacks: Vec<VecDeque<String>> = Vec::new();

    let mut instructions: Vec<Instructions> = Vec::new();

    parse(
        read_to_string("src/test_input.txt")?,
        &mut stacks,
        &mut instructions,
    );

    let mut expected_stacks: Vec<VecDeque<String>> = Vec::new();
    let mut stack1: VecDeque<String> = VecDeque::new();
    stack1.push_back(String::from("N"));
    stack1.push_back(String::from("Z"));
    expected_stacks.push(stack1);
    let mut stack2: VecDeque<String> = VecDeque::new();
    stack2.push_back(String::from("D"));
    stack2.push_back(String::from("C"));
    stack2.push_back(String::from("M"));
    expected_stacks.push(stack2);

    let mut stack3: VecDeque<String> = VecDeque::new();
    stack3.push_back(String::from("P"));
    expected_stacks.push(stack3);

    assert_eq!(stacks, expected_stacks);

    let mut expected_instr: Vec<Instructions> = Vec::new();
    expected_instr.push(Instructions {
        amount: 1,
        from: 2,
        to: 1,
    });
    expected_instr.push(Instructions {
        amount: 3,
        from: 1,
        to: 3,
    });
    expected_instr.push(Instructions {
        amount: 2,
        from: 2,
        to: 1,
    });
    expected_instr.push(Instructions {
        amount: 1,
        from: 1,
        to: 2,
    });

    assert_eq!(instructions, expected_instr);

    Ok(())
}
