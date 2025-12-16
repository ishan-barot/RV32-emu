// simple assembler for rv32i subset

use std::collections::HashMap;

pub struct Assembler {
    labels: HashMap<String, u32>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            labels: HashMap::new(),
        }
    }

    pub fn assemble(&mut self, source: &str) -> Result<Vec<u8>, String> {
        // two-pass assembly: first pass collects labels, second pass generates code
        let lines: Vec<&str> = source.lines().collect();
        
        // pass 1: collect labels
        let mut pc = 0u32;
        let mut cleaned_lines = Vec::new();
        
        for line in &lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if line.ends_with(':') {
                let label = line[..line.len()-1].to_string();
                self.labels.insert(label, pc);
            } else {
                cleaned_lines.push(line);
                pc += 4;
            }
        }
        
        // pass 2: generate code
        let mut code = Vec::new();
        let mut current_pc = 0u32;
        
        for line in cleaned_lines {
            let inst = self.assemble_instruction(line, current_pc)?;
            code.extend_from_slice(&inst.to_le_bytes());
            current_pc += 4;
        }
        
        Ok(code)
    }
    
    fn assemble_instruction(&self, line: &str, pc: u32) -> Result<u32, String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Err("empty instruction".to_string());
        }
        
        let op = parts[0];
        
        match op {
            "add" | "sub" | "and" | "or" | "xor" | "sll" | "srl" | "sra" => {
                self.assemble_rtype(op, &parts[1..])
            }
            "addi" | "andi" | "ori" | "xori" | "slli" | "srli" | "srai" => {
                self.assemble_itype(op, &parts[1..])
            }
            "lw" => self.assemble_load(&parts[1..]),
            "sw" => self.assemble_store(&parts[1..]),
            "beq" | "bne" | "blt" | "bge" => {
                self.assemble_branch(op, &parts[1..], pc)
            }
            "lui" => self.assemble_lui(&parts[1..]),
            "auipc" => self.assemble_auipc(&parts[1..]),
            "jal" => self.assemble_jal(&parts[1..], pc),
            "jalr" => self.assemble_jalr(&parts[1..]),
            _ => Err(format!("unknown instruction: {}", op)),
        }
    }
    
    fn assemble_rtype(&self, op: &str, args: &[&str]) -> Result<u32, String> {
        if args.len() < 3 {
            return Err(format!("not enough args for {}", op));
        }
        
        let rd = parse_reg(args[0])?;
        let rs1 = parse_reg(args[1])?;
        let rs2 = parse_reg(args[2])?;
        
        let (funct3, funct7) = match op {
            "add" => (0x0, 0x00),
            "sub" => (0x0, 0x20),
            "and" => (0x7, 0x00),
            "or" => (0x6, 0x00),
            "xor" => (0x4, 0x00),
            "sll" => (0x1, 0x00),
            "srl" => (0x5, 0x00),
            "sra" => (0x5, 0x20),
            _ => return Err(format!("unknown r-type: {}", op)),
        };
        
        Ok((funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | 0x33)
    }
    
    fn assemble_itype(&self, op: &str, args: &[&str]) -> Result<u32, String> {
        if args.len() < 3 {
            return Err(format!("not enough args for {}", op));
        }
        
        let rd = parse_reg(args[0])?;
        let rs1 = parse_reg(args[1])?;
        let imm = parse_imm(args[2])? & 0xfff;
        
        let funct3 = match op {
            "addi" => 0x0,
            "andi" => 0x7,
            "ori" => 0x6,
            "xori" => 0x4,
            "slli" => 0x1,
            "srli" => 0x5,
            "srai" => 0x5,
            _ => return Err(format!("unknown i-type: {}", op)),
        };
        
        let imm = if op == "srai" {
            imm | 0x400
        } else {
            imm
        };
        
        Ok((imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | 0x13)
    }
    
    fn assemble_load(&self, args: &[&str]) -> Result<u32, String> {
        if args.len() < 2 {
            return Err("not enough args for lw".to_string());
        }
        
        let rd = parse_reg(args[0])?;
        let (imm, rs1) = parse_mem_operand(args[1])?;
        
        Ok(((imm & 0xfff) << 20) | (rs1 << 15) | (0x2 << 12) | (rd << 7) | 0x03)
    }
    
    fn assemble_store(&self, args: &[&str]) -> Result<u32, String> {
        if args.len() < 2 {
            return Err("not enough args for sw".to_string());
        }
        
        let rs2 = parse_reg(args[0])?;
        let (imm, rs1) = parse_mem_operand(args[1])?;
        
        let imm_low = imm & 0x1f;
        let imm_high = (imm >> 5) & 0x7f;
        
        Ok((imm_high << 25) | (rs2 << 20) | (rs1 << 15) | (0x2 << 12) | (imm_low << 7) | 0x23)
    }
    
    fn assemble_branch(&self, op: &str, args: &[&str], pc: u32) -> Result<u32, String> {
        if args.len() < 3 {
            return Err(format!("not enough args for {}", op));
        }
        
        let rs1 = parse_reg(args[0])?;
        let rs2 = parse_reg(args[1])?;
        
        let target = if let Some(addr) = self.labels.get(args[2]) {
            *addr
        } else {
            parse_imm(args[2])? as u32
        };
        
        let offset = target.wrapping_sub(pc);
        
        let imm_12 = (offset >> 12) & 0x1;
        let imm_11 = (offset >> 11) & 0x1;
        let imm_10_5 = (offset >> 5) & 0x3f;
        let imm_4_1 = (offset >> 1) & 0xf;
        
        let funct3 = match op {
            "beq" => 0x0,
            "bne" => 0x1,
            "blt" => 0x4,
            "bge" => 0x5,
            _ => return Err(format!("unknown branch: {}", op)),
        };
        
        Ok((imm_12 << 31) | (imm_10_5 << 25) | (rs2 << 20) | (rs1 << 15) | 
           (funct3 << 12) | (imm_4_1 << 8) | (imm_11 << 7) | 0x63)
    }
    
    fn assemble_lui(&self, args: &[&str]) -> Result<u32, String> {
        if args.is_empty() {
            return Err("not enough args for lui".to_string());
        }
        
        let rd = parse_reg(args[0])?;
        let imm = parse_imm(args[1])? & 0xfffff;
        
        Ok((imm << 12) | (rd << 7) | 0x37)
    }
    
    fn assemble_auipc(&self, args: &[&str]) -> Result<u32, String> {
        if args.len() < 2 {
            return Err("not enough args for auipc".to_string());
        }
        
        let rd = parse_reg(args[0])?;
        let imm = parse_imm(args[1])? & 0xfffff;
        
        Ok((imm << 12) | (rd << 7) | 0x17)
    }
    
    fn assemble_jal(&self, args: &[&str], pc: u32) -> Result<u32, String> {
        if args.len() < 2 {
            return Err("not enough args for jal".to_string());
        }
        
        let rd = parse_reg(args[0])?;
        
        let target = if let Some(addr) = self.labels.get(args[1]) {
            *addr
        } else {
            parse_imm(args[1])? as u32
        };
        
        let offset = target.wrapping_sub(pc);
        
        let imm_20 = (offset >> 20) & 0x1;
        let imm_10_1 = (offset >> 1) & 0x3ff;
        let imm_11 = (offset >> 11) & 0x1;
        let imm_19_12 = (offset >> 12) & 0xff;
        
        Ok((imm_20 << 31) | (imm_19_12 << 12) | (imm_11 << 20) | (imm_10_1 << 21) | 
           (rd << 7) | 0x6f)
    }
    
    fn assemble_jalr(&self, args: &[&str]) -> Result<u32, String> {
        if args.len() < 2 {
            return Err("not enough args for jalr".to_string());
        }
        
        let rd = parse_reg(args[0])?;
        let (imm, rs1) = parse_mem_operand(args[1])?;
        
        Ok(((imm & 0xfff) << 20) | (rs1 << 15) | (rd << 7) | 0x67)
    }
}

fn parse_reg(s: &str) -> Result<u32, String> {
    let s = s.trim_end_matches(',');
    if let Some(stripped) = s.strip_prefix('x') {
        stripped.parse::<u32>()
            .map_err(|_| format!("invalid register: {}", s))
    } else {
        Err(format!("invalid register format: {}", s))
    }
}

fn parse_imm(s: &str) -> Result<u32, String> {
    let s = s.trim_end_matches(',');
    if let Some(hex) = s.strip_prefix("0x") {
        u32::from_str_radix(hex, 16)
            .map_err(|_| format!("invalid hex immediate: {}", s))
    } else {
        s.parse::<i32>()
            .map(|v| v as u32)
            .map_err(|_| format!("invalid immediate: {}", s))
    }
}

fn parse_mem_operand(s: &str) -> Result<(u32, u32), String> {
    // format: offset(reg) e.g. 4(x2)
    if let Some(idx) = s.find('(') {
        let offset_str = &s[..idx];
        let reg_str = &s[idx+1..s.len()-1];
        
        let offset = if offset_str.is_empty() {
            0
        } else {
            parse_imm(offset_str)?
        };
        
        let reg = parse_reg(reg_str)?;
        Ok((offset, reg))
    } else {
        Err(format!("invalid memory operand: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_add() {
        let mut asm = Assembler::new();
        let code = asm.assemble("add x1, x2, x3").unwrap();
        assert_eq!(code.len(), 4);
        let inst = u32::from_le_bytes([code[0], code[1], code[2], code[3]]);
        assert_eq!(inst, 0x003100b3);
    }

    #[test]
    fn test_assemble_with_label() {
        let mut asm = Assembler::new();
        let source = "loop:\naddi x1, x1, 1\nbeq x1, x2, loop";
        let code = asm.assemble(source).unwrap();
        assert_eq!(code.len(), 8);
    }
}
