use clap::{Parser, Subcommand};
use rv32_emu::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rv32-emu")]
#[command(about = "risc-v rv32i emulator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// run a binary or assembly file
    Run {
        /// input file (binary or .s assembly)
        #[arg(short, long)]
        file: PathBuf,
        
        /// load address (default: 0)
        #[arg(short, long, default_value = "0")]
        addr: String,
        
        /// max instructions to execute
        #[arg(short, long, default_value = "1000000")]
        max_steps: usize,
        
        /// show performance metrics
        #[arg(short = 'p', long)]
        perf: bool,
    },
    
    /// assemble a .s file to binary
    Asm {
        /// input assembly file
        #[arg(short, long)]
        input: PathBuf,
        
        /// output binary file
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// run with interactive debugger
    Debug {
        /// input file (binary or .s assembly)
        #[arg(short, long)]
        file: PathBuf,
        
        /// load address (default: 0)
        #[arg(short, long, default_value = "0")]
        addr: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Run { file, addr, max_steps, perf } => {
            run_file(&file, &addr, max_steps, perf);
        }
        Commands::Asm { input, output } => {
            assemble_file(&input, &output);
        }
        Commands::Debug { file, addr } => {
            debug_file(&file, &addr);
        }
    }
}

fn run_file(path: &PathBuf, addr_str: &str, max_steps: usize, show_perf: bool) {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    let data = load_program(path);
    let addr = parse_addr(addr_str).expect("invalid load address");
    
    cpu.load_program(&data, addr);
    cpu.pc = addr;
    
    if show_perf {
        metrics.start();
    }
    
    match exec.run(&mut cpu, &mut metrics, max_steps) {
        Ok(steps) => {
            println!("executed {} instructions", steps);
            if show_perf {
                metrics.print_summary();
            }
        }
        Err(e) => {
            eprintln!("execution error: {}", e);
            std::process::exit(1);
        }
    }
}

fn assemble_file(input: &PathBuf, output: &PathBuf) {
    let source = fs::read_to_string(input)
        .expect("failed to read input file");
    
    let mut asm = assembler::Assembler::new();
    match asm.assemble(&source) {
        Ok(code) => {
            fs::write(output, code)
                .expect("failed to write output file");
            println!("assembled to {}", output.display());
        }
        Err(e) => {
            eprintln!("assembly error: {}", e);
            std::process::exit(1);
        }
    }
}

fn debug_file(path: &PathBuf, addr_str: &str) {
    let mut cpu = cpu::Cpu::new();
    let mut metrics = metrics::Metrics::new();
    let mut dbg = debugger::Debugger::new();
    
    let data = load_program(path);
    let addr = parse_addr(addr_str).expect("invalid load address");
    
    cpu.load_program(&data, addr);
    cpu.pc = addr;
    
    dbg.run(&mut cpu, &mut metrics);
}

fn load_program(path: &PathBuf) -> Vec<u8> {
    if path.extension().and_then(|s| s.to_str()) == Some("s") {
        // assemble on the fly
        let source = fs::read_to_string(path)
            .expect("failed to read assembly file");
        let mut asm = assembler::Assembler::new();
        asm.assemble(&source)
            .expect("failed to assemble")
    } else {
        // TODO: add proper elf32 loader instead of just raw binary
        fs::read(path)
            .expect("failed to read binary file")
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
