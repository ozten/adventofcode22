use std::fs::read_to_string;

use std::error::Error;
use std::ops::RangeInclusive;

fn parse_range(section: &str) -> RangeInclusive<usize> {
    let mut fields = section.split("-");
    match (
        fields.next().unwrap().parse::<usize>(),
        fields.next().unwrap().parse::<usize>(),
    ) {
        (Ok(start), Ok(end)) => RangeInclusive::new(start, end),
        _ => panic!("Unable to parse section {section}"),
    }
}

fn overlap(lines: String) -> usize {
    let mut count = 0;
    for line in lines.split("\n") {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(",");
        let left = parse_range(parts.next().unwrap());
        let right = parse_range(parts.next().unwrap());

        if (left.contains(&right.start()) && left.contains(&right.end()))
            || (right.contains(&left.start()) && right.contains(&left.end()))
        {
            count += 1;
        }
    }
    count
}

#[test]
fn test_overlap() -> Result<(), Box<dyn Error>> {
    assert_eq!(overlap(read_to_string("src/test_input.txt")?), 2);
    Ok(())
}

#[test]
fn test_edge_cases_overlap() -> Result<(), Box<dyn Error>> {
    assert_eq!(overlap(read_to_string("src/edge_case_input.txt")?), 1);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let count = match read_to_string("src/input.txt") {
        Ok(lines) => overlap(lines),
        Err(error) => panic!("Unable to open src/input.txt {:?}", error),
    };
    println!("Answer {count}");
    assert_eq!(count, 644);

    Ok(())
}
