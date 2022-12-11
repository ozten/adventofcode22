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

        loop {
            // A single round

            // let mut enumeration = (&mut self.monkeys).iter().enumerate();

            // for (idx, monkey) in enumeration {
                
                let mut idx = 0;
                while let Some(monkey) = self.monkeys.get_mut(idx) {
                let mut outcomes = monkey.take_turn();
                for outcome in outcomes {
                    self.pass_item(idx, outcome);   
                }
                idx += 1;
            }
            
            println!("After round {round}");
            for monkey in &self.monkeys {
                println!("{:?}", monkey.items);
            }

            round += 1;
            
            if round > 20 {
                break;
            }
            
        }
        
        let mut freq:Vec<isize> = self.monkeys.iter().map(|m| -> isize {
            m.inspect_count as isize
        }).collect();
        freq.sort();

        let l = freq.len();
        freq[l-2..].iter().product()
    }
    pub fn pass_item(&mut self, monkey_1: usize, monkey_2: usize) {
        // give the 0th item from monkey_1 to monkey_2
        let item = self.monkeys.get_mut(monkey_1).unwrap().items.remove(0);
        self.monkeys.get_mut(monkey_2).unwrap().items.push(item);
    }
}
