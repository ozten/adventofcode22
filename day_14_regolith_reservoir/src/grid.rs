use std::fmt::Debug;

use crate::parser::{Coordinate, Line};

pub struct Grid {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize,
    cells: Vec<char>,
}
impl Grid {
    pub fn new(min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> Self {
        let mut cells = Vec::new();
        for _y in 0..=max_y {
            for _x in 0..=max_x {
                cells.push('.');
            }
        }
        Grid {
            min_x,
            max_x,
            min_y,
            max_y,
            cells,
        }
    }

    pub fn record_rock(&mut self, lines: &Vec<Line>) {
        for line in lines {
            println!("Drawing a line from ");
            let mut prev: Option<&Coordinate> = None;

            for next in line {
                match prev {
                    Some(prev) => {
                        println!("Processing from {:?} to {:?}", prev, next);

                        let y_range = if next.y < prev.y {
                            next.y..=prev.y
                        } else {
                            prev.y..=next.y
                        };

                        for y in y_range {
                            let x_range = if next.x < prev.x {
                                next.x..=prev.x
                            } else {
                                prev.x..=next.x
                            };
                            for x in x_range {
                                let idx = self.cell_idx(&x, &y);
                                self.cells[idx] = '#';
                                println!("Drawing a line at {:?}, {:?} ", x, y);
                            }
                        }
                    }
                    None => {}
                }

                prev = Some(next);
            }
        }
    }

    pub fn clear_sand(&mut self, sand: &Coordinate) {
        let idx = self.cell_idx(&sand.x, &sand.y);
        self.cells[idx] = '.';
    }

    pub fn record_sand(&mut self, x: usize, y: usize) {
        let idx = self.cell_idx(&(x as isize), &(y as isize));
        self.cells[idx] = 'o';
    }

    pub fn cell_idx(&self, x: &isize, y: &isize) -> usize {
        (self.max_x as usize * *y as usize) + *x as usize
    }

    pub fn is_ouside_grid(&self, x: isize, y: isize) -> bool {
        x < self.min_x || 
        x > self.max_x || 
        y < self.min_y || y > self.max_y
    }

    pub fn can_move_down(&self, sand: &Coordinate) -> bool {
        let idx = self.cell_idx(&sand.x, &(sand.y + 1));
        match self.cells[idx] {
            '.' => true,
            _ => false,
        }
    }
    pub fn can_move_down_left(&self, sand: &Coordinate) -> bool {
        let idx = self.cell_idx(&(sand.x - 1), &(sand.y + 1));
        println!("{:?} {:?}", sand, self.cells[idx]);
        match self.cells[idx] {
            '.' => true,
            _ => false,
        }
    }
    pub fn can_move_down_right(&self, sand: &Coordinate) -> bool {
        let idx = self.cell_idx(&(sand.x + 1), &(sand.y + 1));        
        match self.cells[idx] {
            '.' => true,
            _ => false,
        }
    }
}

// TODO: Question - Why can't I use println!("{}", grid)... it says Debug is not implemented...
// but I can use {:?}... WTF?
impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::from("");

        buf += &String::from("  4     5  5\n");

        buf += &String::from("  9     0  0\n");
        buf += &String::from("  4     0  3\n");
        // TODO: grid is 0 based or should we use min x and min y?
        for y in 0..=self.max_y {
            if y < self.min_y || y > self.max_y {
                println!("Skipping y={y} out of range {} {}", self.min_y, self.max_y);
                continue;
            }
            buf += &String::from(format!("{y} "));
            for x in 0..=self.max_x {
                if x < self.min_x - 1 || x > self.max_x {
                    // println!("Skipping x={x} out of range {} {}", self.min_x, self.max_x);
                    continue;
                }
                let c = self
                    .cells
                    .get((y as usize * self.max_x as usize) + x as usize)
                    .unwrap();
                buf += &String::from(*c);
            }
            buf += &String::from("\n");
        }

        f.write_str(&buf)
        // f.debug_struct("Grid").field("min_x", &self.min_x).field("max_x", &self.max_x).field("min_y", &self.min_y).field("max_y", &self.max_y).field("cells", &self.cells).finish()
    }
}

pub fn min_max(lines: &Vec<Line>, sand: &Coordinate) -> ((isize, isize), (isize, isize)) {
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;

    for line in lines {
        for coord in line {
            update_min_max(&mut min_x, &mut max_x, &mut min_y, &mut max_y, &coord);
        }
    }
    update_min_max(&mut min_x, &mut max_x, &mut min_y, &mut max_y, sand);

    ((min_x, max_x), (min_y, max_y))
}

fn update_min_max(
    min_x: &mut isize,
    max_x: &mut isize,
    min_y: &mut isize,
    max_y: &mut isize,
    coord: &Coordinate,
) {
    update_min(min_x, &coord.x);
    update_max(max_x, &coord.x);
    update_min(min_y, &coord.y);
    update_max(max_y, &coord.y);
}

fn update_min(min_x: &mut isize, x: &isize) {
    if min_x as &isize > x {
        *min_x = *x;
    }
}

fn update_max(max_x: &mut isize, x: &isize) {
    if (max_x as &isize) < x {
        *max_x = *x;
    }
}
