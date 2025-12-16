// interactive debugger

use crate::cpu::Cpu;
use crate::decoder::Instruction;
use crate::executor::Executor;
use crate::metrics::Metrics;
use std::collections::HashSet;
use std::io::{self, Write};

pub struct Debugger {
    breakpoints: HashSet<u32>,
    pub executor: Executor,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: HashSet::new(),
            executor: Executor::new(),
        }
    }

    pub fn run(&mut self, cpu: &mut Cpu, metrics: &mut Metrics) {
        println!("debugger started. type 'help' for commands");
        
        loop {
            print!("(dbg) ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }
            
            let input = input.trim();
            if input.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = input.split_whitespace().collect();
            let cmd = parts[0];
            
            match cmd {
                "help" | "h" => self.print_help(),
                "step" | "s" => self.step(cpu, metrics),
                "continue" | "c" => {
                    if self.continue_exec(cpu, metrics) {
                        break;
                    }
                }
                "break" | "b" => self.set_breakpoint(&parts[1..]),
                "regs" | "r" => self.dump_regs(cpu),
                "mem" | "m" => self.dump_mem(cpu, &parts[1..]),
                "dis" | "d" => self.disassemble(cpu, &parts[1..]),
                "pc" => println!("pc = 0x{:08x}", cpu.pc),
                "quit" | "q" => break,
                _ => println!("unknown command: {}", cmd),
            }
        }
    }
    
    fn print_help(&self) {
        println!("commands:");
        println!("  step (s)         - execute one instruction");
        println!("  continue (c)     - continue execution until breakpoint");
        println!("  break (b) <addr> - set breakpoint at address");
        println!("  regs (r)         - dump register file");
        println!("  mem (m) <addr>   - dump memory at address");
        println!("  dis (d) [addr]   - disassemble instructions");
        println!("  pc               - show program counter");
        println!("  quit (q)         - exit debugger");
    }
    
    fn step(&mut self, cpu: &mut Cpu, metrics: &mut Metrics) {
        let pc_before = cpu.pc;
        match self.executor.step(cpu, metrics) {
            Ok(_) => {
                let raw = cpu.read_word(pc_before);
                let inst = Instruction::decode(raw);
                println!("0x{:08x}: {}", pc_before, inst.disassemble());
            }
            Err(e) => println!("error: {}", e),
        }
    }
    
    fn continue_exec(&mut self, cpu: &mut Cpu, metrics: &mut Metrics) -> bool {
        loop {
            if self.breakpoints.contains(&cpu.pc) {
                println!("hit breakpoint at 0x{:08x}", cpu.pc);
                return false;
            }
            
            match self.executor.step(cpu, metrics) {
                Ok(_) => {},
                Err(e) => {
                    println!("stopped: {}", e);
                    return true;
                }
            }
        }
    }
    
    fn set_breakpoint(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!("usage: break <address>");
            return;
        }
        
        if let Ok(addr) = parse_addr(args[0]) {
            self.breakpoints.insert(addr);
            println!("breakpoint set at 0x{:08x}", addr);
        } else {
            println!("invalid address: {}", args[0]);
        }
    }
    
    fn dump_regs(&self, cpu: &Cpu) {
        println!("registers:");
        for i in 0..32 {
            if i % 4 == 0 && i > 0 {
                println!();
            }
            print!("  x{:<2} = 0x{:08x}", i, cpu.regs[i]);
        }
        println!();
        println!("  pc  = 0x{:08x}", cpu.pc);
    }
    
    fn dump_mem(&self, cpu: &Cpu, args: &[&str]) {
        if args.is_empty() {
            println!("usage: mem <address> [count]");
            return;
        }
        
        let addr = match parse_addr(args[0]) {
            Ok(a) => a,
            Err(_) => {
                println!("invalid address: {}", args[0]);
                return;
            }
        };
        
        let count = if args.len() > 1 {
            args[1].parse::<usize>().unwrap_or(16)
        } else {
            16
        };
        
        println!("memory at 0x{:08x}:", addr);
        for i in 0..count {
            let a = addr + (i * 4) as u32;
            if a as usize + 4 <= cpu.mem.len() {
                let val = cpu.read_word(a);
                println!("  0x{:08x}: 0x{:08x}", a, val);
            }
        }
    }
    
    fn disassemble(&self, cpu: &Cpu, args: &[&str]) {
        let addr = if args.is_empty() {
            cpu.pc
        } else {
            match parse_addr(args[0]) {
                Ok(a) => a,
                Err(_) => {
                    println!("invalid address: {}", args[0]);
                    return;
                }
            }
        };
        
        println!("disassembly at 0x{:08x}:", addr);
        for i in 0..10 {
            let a = addr + (i * 4);
            if a as usize + 4 <= cpu.mem.len() {
                let raw = cpu.read_word(a);
                let inst = Instruction::decode(raw);
                let marker = if a == cpu.pc { "=>" } else { "  " };
                println!("  {} 0x{:08x}: {}", marker, a, inst.disassemble());
            }
        }
    }
}

fn parse_addr(s: &str) -> Result<u32, String> {
    if let Some(hex) = s.strip_prefix("0x") {
        u32::from_str_radix(hex, 16)
            .map_err(|_| format!("invalid hex address: {}", s))
    } else {
        s.parse::<u32>()
            .map_err(|_| format!("invalid address: {}", s))
    }
}
