use std::fs::read_to_string;

use crate::monkey::Monkey;
use crate::monkey::Operation::{Add, Mul, Square};

pub fn parse(filename: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let last_idx = if monkeys.len() == 0 { 0 } else { monkeys.len() -1};
        if !line.is_empty() {
            match line {
                line if line.starts_with("Monkey ") => {
                    monkeys.push(Monkey::new())
                }
                //   Starting items: 79, 98
                line if line.starts_with("  Starting items: ") => {
                    let mut parts = line.split(":");
                    parts.next();
                    let numbers = parts.next().unwrap();
                    
                    monkeys.get_mut(last_idx).unwrap().set_items(
                        numbers.split(", ").map(|part| -> isize {
                            part.trim().parse().unwrap()                            
                        }).collect()
                    );
                    
                },
                line if line.starts_with("  Operation: new = old * old") => {
                    monkeys.get_mut(last_idx).unwrap().set_operation(Square, 0);
                },
                line if line.starts_with("  Operation: new = old ") => {
                    let mut parts = line[23..].split(" ");
                    let op = match parts.next() {
                        Some(x) if x == "*" => Mul,
                        Some(x) if x == "+" => Add,
                        Some(x) => panic!("Unknown op {line} {:?}", x),
                        None => panic!("Problem parsing ops")
                    };
                    let op_val:isize = parts.next().unwrap().parse().unwrap();
                    monkeys.get_mut(last_idx).unwrap().set_operation(op, op_val);

                },
                // Test: divisible by 17
                line if line.starts_with("  Test: divisible by ") => {
                    let div: isize = line[21..].parse().unwrap();
                    monkeys.get_mut(last_idx).unwrap().set_divisible(div);
                },
                //     If true: throw to monkey 0
                line if line.starts_with("    If true") => {
                    let monkey_idx = line[29..].parse().unwrap();
                    monkeys.get_mut(last_idx).unwrap().set_true_monkey(monkey_idx);
                }
                //    If false: throw to monkey 1
                line if line.starts_with("    If false") => {
                    let monkey_idx = line[30..].parse().unwrap();
                    monkeys.get_mut(last_idx).unwrap().set_false_monkey(monkey_idx);
                }
                _ => panic!("Unexpected line _{line}_"),
            }
        }
    }
    monkeys
    
}


#[cfg(test)]
mod test {
    
    use std::io::Error;

    use crate::parser::parse;

    #[test]
    fn test_parse() -> Result<(), Error> {
        let instructions = parse("./src/test_input.txt");

        
        Ok(())
    }
}
