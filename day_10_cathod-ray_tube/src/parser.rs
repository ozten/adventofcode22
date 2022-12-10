use std::fs::read_to_string;

use crate::cpu::CpuInstructions;
use crate::cpu::CpuInstructions::{AddX, Noop};


pub fn parse(filename: &str) -> Vec<CpuInstructions> {
    let mut instructions: Vec<CpuInstructions> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        if !line.is_empty() {
            instructions.push(parse_line(line));
        }
    }
    instructions
}

fn parse_line(input: &str) -> CpuInstructions {
    match input {
        input if input.starts_with("addx -") => {
            let value: isize = input[6..].parse().unwrap();
            AddX { value: 0 - value, cost: 2 }
        }
        input if input.starts_with("addx ") => AddX {
            value: input[5..].parse().unwrap(),
            cost: 2
        },
        input if input == "noop" => Noop { cost: 1 },
        _ => panic!("Unexpected line _{input}_"),
    }
}

#[cfg(test)]
mod test {
    
    use std::io::Error;

    use crate::cpu::CpuInstructions::{AddX, Noop};
    use crate::parser::parse;

    #[test]
    fn test_parse() -> Result<(), Error> {
        let instructions = parse("./src/test_input.txt");

        assert_eq!(instructions.get(0), Some(&AddX { value: 15, cost: 2 }));
        assert_eq!(instructions.get(9), Some(&Noop { cost: 1 }));
        Ok(())
    }
}
