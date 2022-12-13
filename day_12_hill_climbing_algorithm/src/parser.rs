use std::fs::read_to_string;

use nom::IResult;
use nom::number::complete::be_u16;
use nom::bytes::complete::take;

use crate::grid::{Grid, Space, Path};
use crate::point::Point;

pub fn parse(filename: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;

    let mut start:Option<Point> = None;
    let mut goal:Option<Point> = None;

    let mut spaces: Vec<Space> = Vec::new();


    let mut x = 0;
    let mut y = 0;

    for line in read_to_string(filename).unwrap().lines() {
        println!("y={y} starting a new line");
        x = 0;

        if width == 0 {
            width = line.len();
        }
        if !line.is_empty() {
            height += 1;

            println!("x={x} starting a new char");

            for ch in line.chars() {
                if 'a' <= ch && ch <= 'z' {
                    println!("Lower {ch}");
                    spaces.push(Space::Element { path: Path { value: ch } });
                } else if 'S' == ch {
                    println!("Starting {x},{y}");
                    start = Some(Point::new(x, y));
                    spaces.push(Space::Start);
                } else if 'E' == ch {
                    println!("End Goal {x}, {y}");
                    goal = Some(Point::new(x, y));
                    spaces.push(Space::End);
                } else {
                    println!("EOL");
                }
                x += 1;
            }
            y += 1;
        }
    }



    println!("Width={width} Height={height} Start Goal");
    Grid { width, height, spaces, start: start.unwrap(), goal: goal.unwrap() }
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
