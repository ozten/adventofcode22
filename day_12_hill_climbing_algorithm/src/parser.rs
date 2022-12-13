use std::fs::read_to_string;

use crate::grid::{Grid, Path, Space};
use crate::point::Point;

pub fn parse(filename: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;

    let mut start: Option<Point> = None;
    let mut goal: Option<Point> = None;

    let mut spaces: Vec<Space> = Vec::new();

    let mut x;
    let mut y = 0;

    for line in read_to_string(filename).unwrap().lines() {
        x = 0;

        if width == 0 {
            width = line.len();
        }
        if !line.is_empty() {
            height += 1;

            for ch in line.chars() {
                if 'a' <= ch && ch <= 'z' {
                    spaces.push(Space::Element {
                        path: Path { value: ch },
                    });
                } else if 'S' == ch {
                    start = Some(Point::new(x, y));
                    spaces.push(Space::Start);
                } else if 'E' == ch {
                    goal = Some(Point::new(x, y));
                    spaces.push(Space::End);
                }
                x += 1;
            }
            y += 1;
        }
    }
    Grid {
        width,
        height,
        spaces,
        start: start.unwrap(),
        goal: goal.unwrap(),
    }
}
