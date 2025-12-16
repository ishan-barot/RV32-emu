// instruction execution

use crate::cpu::Cpu;
use crate::decoder::{Instruction, Opcode};
use crate::metrics::Metrics;

pub struct Executor {
    pub halted: bool,
}

impl Executor {
    pub fn new() -> Self {
        Executor { halted: false }
    }

    pub fn step(&mut self, cpu: &mut Cpu, metrics: &mut Metrics) -> Result<(), String> {
        if self.halted {
            return Err("cpu halted".to_string());
        }

        let raw = cpu.read_word(cpu.pc);
        let inst = Instruction::decode(raw);
        metrics.record_instruction(&inst);

        match inst.opcode {
            Opcode::Add => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                cpu.write_reg(inst.rd, rs1.wrapping_add(rs2));
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Sub => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                cpu.write_reg(inst.rd, rs1.wrapping_sub(rs2));
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::And => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                cpu.write_reg(inst.rd, rs1 & rs2);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Or => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                cpu.write_reg(inst.rd, rs1 | rs2);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Xor => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                cpu.write_reg(inst.rd, rs1 ^ rs2);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Sll => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                let shamt = rs2 & 0x1f;
                cpu.write_reg(inst.rd, rs1 << shamt);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Srl => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                let shamt = rs2 & 0x1f;
                cpu.write_reg(inst.rd, rs1 >> shamt);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Sra => {
                let rs1 = cpu.read_reg(inst.rs1) as i32;
                let rs2 = cpu.read_reg(inst.rs2);
                let shamt = rs2 & 0x1f;
                cpu.write_reg(inst.rd, (rs1 >> shamt) as u32);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Addi => {
                let rs1 = cpu.read_reg(inst.rs1);
                cpu.write_reg(inst.rd, rs1.wrapping_add(inst.imm as u32));
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Andi => {
                let rs1 = cpu.read_reg(inst.rs1);
                cpu.write_reg(inst.rd, rs1 & (inst.imm as u32));
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Ori => {
                let rs1 = cpu.read_reg(inst.rs1);
                cpu.write_reg(inst.rd, rs1 | (inst.imm as u32));
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Xori => {
                let rs1 = cpu.read_reg(inst.rs1);
                cpu.write_reg(inst.rd, rs1 ^ (inst.imm as u32));
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Slli => {
                let rs1 = cpu.read_reg(inst.rs1);
                let shamt = (inst.imm & 0x1f) as u32;
                cpu.write_reg(inst.rd, rs1 << shamt);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Srli => {
                let rs1 = cpu.read_reg(inst.rs1);
                let shamt = (inst.imm & 0x1f) as u32;
                cpu.write_reg(inst.rd, rs1 >> shamt);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Srai => {
                let rs1 = cpu.read_reg(inst.rs1) as i32;
                let shamt = (inst.imm & 0x1f) as u32;
                cpu.write_reg(inst.rd, (rs1 >> shamt) as u32);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Lw => {
                let rs1 = cpu.read_reg(inst.rs1);
                let addr = rs1.wrapping_add(inst.imm as u32);
                let val = cpu.read_word(addr);
                cpu.write_reg(inst.rd, val);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Sw => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                let addr = rs1.wrapping_add(inst.imm as u32);
                cpu.write_word(addr, rs2);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Beq => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                if rs1 == rs2 {
                    cpu.pc = cpu.pc.wrapping_add(inst.imm as u32);
                    metrics.record_branch(true);
                } else {
                    cpu.pc = cpu.pc.wrapping_add(4);
                    metrics.record_branch(false);
                }
            }
            Opcode::Bne => {
                let rs1 = cpu.read_reg(inst.rs1);
                let rs2 = cpu.read_reg(inst.rs2);
                if rs1 != rs2 {
                    cpu.pc = cpu.pc.wrapping_add(inst.imm as u32);
                    metrics.record_branch(true);
                } else {
                    cpu.pc = cpu.pc.wrapping_add(4);
                    metrics.record_branch(false);
                }
            }
            Opcode::Blt => {
                let rs1 = cpu.read_reg(inst.rs1) as i32;
                let rs2 = cpu.read_reg(inst.rs2) as i32;
                if rs1 < rs2 {
                    cpu.pc = cpu.pc.wrapping_add(inst.imm as u32);
                    metrics.record_branch(true);
                } else {
                    cpu.pc = cpu.pc.wrapping_add(4);
                    metrics.record_branch(false);
                }
            }
            Opcode::Bge => {
                let rs1 = cpu.read_reg(inst.rs1) as i32;
                let rs2 = cpu.read_reg(inst.rs2) as i32;
                if rs1 >= rs2 {
                    cpu.pc = cpu.pc.wrapping_add(inst.imm as u32);
                    metrics.record_branch(true);
                } else {
                    cpu.pc = cpu.pc.wrapping_add(4);
                    metrics.record_branch(false);
                }
            }
            Opcode::Lui => {
                cpu.write_reg(inst.rd, inst.imm as u32);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Auipc => {
                let val = cpu.pc.wrapping_add(inst.imm as u32);
                cpu.write_reg(inst.rd, val);
                cpu.pc = cpu.pc.wrapping_add(4);
            }
            Opcode::Jal => {
                let link = cpu.pc.wrapping_add(4);
                cpu.write_reg(inst.rd, link);
                cpu.pc = cpu.pc.wrapping_add(inst.imm as u32);
            }
            Opcode::Jalr => {
                let rs1 = cpu.read_reg(inst.rs1);
                let link = cpu.pc.wrapping_add(4);
                cpu.write_reg(inst.rd, link);
                // fix: jalr must clear bit 0 per spec
                cpu.pc = (rs1.wrapping_add(inst.imm as u32)) & !1;
            }
            Opcode::Unknown => {
                return Err(format!("unknown instruction at pc=0x{:x}", cpu.pc));
            }
        }

        Ok(())
    }

    pub fn run(&mut self, cpu: &mut Cpu, metrics: &mut Metrics, max_steps: usize) -> Result<usize, String> {
        let mut steps = 0;
        while steps < max_steps {
            if let Err(e) = self.step(cpu, metrics) {
                return Err(e);
            }
            steps += 1;
            
            // simple halt detection: if we're stuck in a tight loop at same pc
            // this is kind of hacky but works for most test cases
            // TODO: add proper ecall-based halt mechanism
            if cpu.pc == 0 {
                self.halted = true;
                break;
            }
        }
        Ok(steps)
    }
}
