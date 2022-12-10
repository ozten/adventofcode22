use crate::cpu::{Cpu, CpuInstructions};

#[derive(Debug, PartialEq)]
pub struct CathodeRayTube {
    cpu: Cpu,
    cycle: usize,
}

impl CathodeRayTube {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            cycle: 1,
        }
    }

    pub fn simulate(&mut self, instructions: &Vec<CpuInstructions>) {
        let mut instr = instructions.iter();
        let mut current_instr = instr.next().unwrap();
        let mut current_instr_cost = match current_instr {
            CpuInstructions::AddX { cost, .. } => cost.clone(),
            CpuInstructions::Noop { cost } => cost.clone(),
        };
        let mut sum_strength = 0;

        loop {
            println!(
                "Starting cycle {} CPU X={} current instruction={:?} instruction cost={:?}",
                self.cycle, self.cpu.x, &current_instr, &current_instr_cost
            );

            if self.cycle as f32 / 20.0 == 1.0 ||
            self.cycle as f32 / 60.0 == 1.0 ||
            self.cycle as f32 / 100.0 == 1.0 ||
            self.cycle as f32 / 140.0 == 1.0 ||
            self.cycle as f32 / 180.0 == 1.0 ||

            self.cycle as f32 / 220.0 == 1.0
            
            /*
                    || (self.cycle / 60) == 1
                    || (self.cycle / 100) == 1
                    || (self.cycle / 140) == 1
                    || (self.cycle / 180) == 1
                    || (self.cycle / 220) == 1 */
                {

                    println!(
                        "{} * {} = Frequency {}",
                        self.cycle,
                        self.cpu.x,
                        self.cycle as isize * self.cpu.x
                    );
                    sum_strength += self.cycle as isize * self.cpu.x;
                    println!("Freq Sum: {}", sum_strength);
                }

            current_instr_cost -= 1;
            if current_instr_cost == 0 {
                println!("Executing the current instruction");
                match current_instr {
                    CpuInstructions::AddX { value, .. } => {
                        self.cpu.x += value;
                    }
                    CpuInstructions::Noop { .. } => {}
                }
                
                println!("Cpu.x={}", self.cpu.x);
                match instr.next() {
                    Some(i) => {
                        current_instr = i;
                        current_instr_cost = cost(i);
                    }
                    None => break,
                }
            }

            self.cycle += 1;
        }
    }
}

fn cost(instruction: &CpuInstructions) -> usize {
    match instruction {
        CpuInstructions::AddX { cost, .. } => cost.clone(),
        CpuInstructions::Noop { cost } => cost.clone(),
    }
}

#[test]
fn test_math() {
    let actual: f32 = 21 as f32 / 20 as f32;
    // assert_eq!((21 as f32) / (20  as f32), 1);
}