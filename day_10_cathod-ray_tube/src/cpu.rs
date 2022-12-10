#[derive(Debug, PartialEq)]
pub enum CpuInstructions {
    AddX { value: isize, cost: usize },
    Noop { cost: usize }
}

#[derive(Debug, PartialEq)]
pub struct Cpu {
    pub x: isize
}

impl Cpu {
    pub fn new() -> Self {
        Self { x: 1 }
    }
}
