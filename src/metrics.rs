// performance metrics tracking

use crate::decoder::Instruction;
use std::collections::HashMap;
use std::time::Instant;

pub struct Metrics {
    pub inst_count: u64,
    pub inst_mix: HashMap<String, u64>,
    pub branch_taken: u64,
    pub branch_not_taken: u64,
    start_time: Option<Instant>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            inst_count: 0,
            inst_mix: HashMap::new(),
            branch_taken: 0,
            branch_not_taken: 0,
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn record_instruction(&mut self, inst: &Instruction) {
        self.inst_count += 1;
        let name = format!("{:?}", inst.opcode);
        *self.inst_mix.entry(name).or_insert(0) += 1;
    }

    pub fn record_branch(&mut self, taken: bool) {
        if taken {
            self.branch_taken += 1;
        } else {
            self.branch_not_taken += 1;
        }
    }

    pub fn mips(&self) -> f64 {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                return (self.inst_count as f64) / elapsed / 1_000_000.0;
            }
        }
        0.0
    }

    pub fn print_summary(&self) {
        println!("\nperformance metrics:");
        println!("  instructions executed: {}", self.inst_count);
        println!("  mips: {:.2}", self.mips());
        
        if self.branch_taken + self.branch_not_taken > 0 {
            let total_branches = self.branch_taken + self.branch_not_taken;
            let taken_pct = (self.branch_taken as f64 / total_branches as f64) * 100.0;
            println!("\nbranch statistics:");
            println!("  taken: {} ({:.1}%)", self.branch_taken, taken_pct);
            println!("  not taken: {} ({:.1}%)", self.branch_not_taken, 100.0 - taken_pct);
        }

        if !self.inst_mix.is_empty() {
            println!("\ninstruction mix:");
            let mut sorted: Vec<_> = self.inst_mix.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));
            for (name, count) in sorted.iter().take(10) {
                let pct = (**count as f64 / self.inst_count as f64) * 100.0;
                println!("  {:<12} {:>8} ({:.1}%)", name, count, pct);
            }
        }
    }
}
