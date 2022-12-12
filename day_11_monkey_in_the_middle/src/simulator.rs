use crate::monkey::Monkey;

#[derive(Debug)]
pub struct Simulator {
    monkeys: Vec<Monkey>,
}

impl Simulator {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }
    pub fn simulate(&mut self) -> isize {
        let mut round = 1;
        let modulo = self
            .monkeys
            .iter()
            .map(|m| -> isize { m.divisible })
            .product();

        loop {
            let mut idx = 0;
            while let Some(monkey) = self.monkeys.get_mut(idx) {
                let mut outcomes = monkey.take_turn(modulo);
                for outcome in outcomes {
                    self.pass_item(idx, outcome);
                }
                idx += 1;
            }

            /* Part 1
            println!("After round {round}");
            for monkey in &self.monkeys {
                println!("{:?}", monkey.items);
            } */

            round += 1;

            // Part 1
            // if round > 20 {
            if round > 10_000 {
                break;
            }
        }

        let mut freq: Vec<isize> = self
            .monkeys
            .iter()
            .map(|m| -> isize { m.inspect_count as isize })
            .collect();
        freq.sort();
        println!("{:?}", freq);
        let l = freq.len();
        freq[l - 2..].iter().product()
    }
    pub fn pass_item(&mut self, monkey_1: usize, monkey_2: usize) {
        // give the 0th item from monkey_1 to monkey_2
        let item = self.monkeys.get_mut(monkey_1).unwrap().items.remove(0);
        self.monkeys.get_mut(monkey_2).unwrap().items.push(item);
    }
}
