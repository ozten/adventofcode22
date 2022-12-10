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
        let part_1 = false;

        loop {
            let col = (self.cycle as isize - 1) % 40;
            if self.cpu.x as isize - 1 <= col && col <= self.cpu.x as isize + 1 {
                print!("#");
            } else {
                print!(".");
            }

            if col == 39 {
                println!("");
            }

            if self.cycle == 20
                || self.cycle == 60
                || self.cycle == 100
                || self.cycle == 140
                || self.cycle == 180
                || self.cycle == 220
            {
                sum_strength += self.cycle as isize * self.cpu.x;
                if part_1 {
                    println!("Freq Sum: {}", sum_strength);    
                }
                
            }

            current_instr_cost -= 1;
            if current_instr_cost == 0 {
                match current_instr {
                    CpuInstructions::AddX { value, .. } => {
                        self.cpu.x += value;
                    }
                    CpuInstructions::Noop { .. } => {}
                }

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
