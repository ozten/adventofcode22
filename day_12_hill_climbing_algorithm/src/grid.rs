use crate::point::Point;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Path {
    pub value: char,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Space {
    Element { path: Path },
    Start,
    End,
}

impl Space {
    pub fn can_move(&self, to: &Space) -> bool {
        match self {
            Space::Start => true,
            // End of the road!
            Space::Element { path } if path.value == 'z' => true,
            Space::Element { path } => {
                let from_value = path.value as u32;
                match to {
                    // Approachable
                    Space::Element { path } if from_value + 1 >= path.value as u32 => true,
                    // Too steep for remaining Space::Element { path }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub spaces: Vec<Space>,
    pub start: Point,
    pub goal: Point,
}

impl Grid {
    fn idx(&self, point: &Point) -> usize {
        self.width * point.y + point.x
    }

    fn get_by_row(&self, point: &Point, y: isize) -> &Space {
        let new_y = point.y as isize + y;
        self.spaces
            .get(self.width * (new_y as usize) + point.x)
            .unwrap()
    }

    fn get_by_col(&self, point: &Point, x: isize) -> &Space {
        let new_x = point.x as isize + x;
        self.spaces
            .get(self.width * point.y + new_x as usize)
            .unwrap()
    }

    pub fn valid_neighbors(&self, point: &Point) -> Vec<(Point, usize)> {
        let mut valid = Vec::new();
        let cur = self.spaces.get(self.idx(point)).unwrap();
        // up
        if point.y >= 1 {
            let up = self.get_by_row(point, -1);
            if cur.can_move(&up) {
                valid.push(Point {
                    x: point.x,
                    y: point.y - 1,
                });
            }
        }

        // right
        if point.x + 1 < self.width {
            let right = self.get_by_col(point, 1);
            if cur.can_move(&right) {
                valid.push(Point {
                    x: point.x + 1,
                    y: point.y,
                });
            }
        }

        // down
        if point.y + 1 < self.height {
            let down = self.get_by_row(point, 1);
            if cur.can_move(&down) {
                valid.push(Point {
                    x: point.x,
                    y: point.y + 1,
                });
            }
        }

        // left
        if point.x >= 1 {
            let left = self.get_by_col(point, -1);
            if cur.can_move(&left) {
                valid.push(Point {
                    x: point.x - 1,
                    y: point.y,
                });
            }
        }
        valid.into_iter().map(|p| (p, 1)).collect()
    }
}
