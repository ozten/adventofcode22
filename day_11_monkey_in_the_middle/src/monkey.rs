

#[derive(Debug)]
pub enum Operation {
    Mul,
    Square,
    Add
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<isize>,
    operation: Operation,
    op_value: isize,
    divisible: isize,
    true_monkey: usize,
    false_monkey: usize,
    pub inspect_count: usize,

}

impl Monkey {
    pub fn new() -> Self {
        Self { items: Vec::new(), operation: Operation::Mul, op_value: 0, divisible: 1, true_monkey: 0, false_monkey: 1,inspect_count: 0 }
    }
    
    pub fn set_items(&mut self, items: Vec<isize>) {
        self.items = items;
    }
    pub fn set_operation(&mut self, operation: Operation, op_value: isize) {
        self.operation = operation;
        self.op_value = op_value;
    }
    pub fn set_divisible(&mut self, divisible: isize) {
        self.divisible = divisible;
    }

    pub fn set_true_monkey(&mut self, monkey: usize) {
        self.true_monkey = monkey;
    }
    pub fn set_false_monkey(&mut self, monkey: usize) {
        self.false_monkey = monkey;
    }

    pub fn take_turn(&mut self) -> Vec<usize> {
        let mut outcomes:Vec<usize> = Vec::new();
        let mut new_items: Vec<isize> = Vec::new();
        for item in &self.items {
            self.inspect_count += 1;            
            let mut worry_level = match &self.operation {
                Operation::Add => item + self.op_value,
                Operation::Mul => item * self.op_value,
                Operation::Square => item * item,
            };
            worry_level = worry_level / 3;
            new_items.push(worry_level);
            if worry_level % self.divisible == 0 {
                outcomes.push(self.true_monkey);
            } else {
                outcomes.push(self.false_monkey);
            }
        }
        self.items = new_items;
        outcomes 
    }
}

#[test]
fn test_take_turn() {
    let mut monkey0 = Monkey { items: vec![79, 98], operation: Operation::Mul, op_value: 19, divisible: 23, true_monkey: 2, false_monkey: 3, inspect_count: 0};
    let outcomes = monkey0.take_turn();
    assert_eq!(outcomes, vec![3, 3]);
}