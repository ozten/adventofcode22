use std::fs::read_to_string;

#[derive(Debug)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn move_to(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }
}

pub type Line = Vec<Coordinate>;

pub fn parse(filename: &str) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let data = read_to_string(filename).expect("Unable to read file input {filename}");
    for datum in data.lines() {
        if !datum.is_empty() {
            let mut line: Line = Vec::new();
            let parts = datum.split("->");
            for part in parts {
                let mut parts = part.split(",");
                let x: isize = parts
                    .next()
                    .expect("Coordinate and line data malformed")
                    .trim()
                    .parse()
                    .expect("Unable to parse x coordinate");
                let y: isize = parts
                    .next()
                    .expect("Coordinate and line data malformed")
                    .trim()
                    .parse()
                    .expect("Unable to parse y coordinate");
                line.push(Coordinate { x, y });
            }
            lines.push(line);
        }
    }
    lines
}
