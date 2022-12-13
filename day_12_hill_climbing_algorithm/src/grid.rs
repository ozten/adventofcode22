use crate::point::Point;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Path {
    pub value: char
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Space {
    Element { path: Path },
    Start,
    End
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub spaces: Vec<Space>,
    pub start: Point,
    pub goal: Point,
}
/*
impl Grid {
    fn new() -> Self {

    }
}
 */