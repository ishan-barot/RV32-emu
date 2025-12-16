// instruction decode logic

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    // r-type
    Add, Sub, And, Or, Xor, Sll, Srl, Sra,
    // i-type
    Addi, Andi, Ori, Xori, Slli, Srli, Srai,
    Lw, Jalr,
    // s-type  
    Sw,
    // b-type
    Beq, Bne, Blt, Bge,
    // u-type
    Lui, Auipc,
    // j-type
    Jal,
    // unknown
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub imm: i32,
}

impl Instruction {
    pub fn decode(raw: u32) -> Self {
        let opcode_bits = raw & 0x7f;
        let rd = ((raw >> 7) & 0x1f) as usize;
        let funct3 = (raw >> 12) & 0x7;
        let rs1 = ((raw >> 15) & 0x1f) as usize;
        let rs2 = ((raw >> 20) & 0x1f) as usize;
        let funct7 = (raw >> 25) & 0x7f;

        match opcode_bits {
            0x33 => {
                // r-type
                let opcode = match (funct3, funct7) {
                    (0x0, 0x00) => Opcode::Add,
                    (0x0, 0x20) => Opcode::Sub,
                    (0x7, 0x00) => Opcode::And,
                    (0x6, 0x00) => Opcode::Or,
                    (0x4, 0x00) => Opcode::Xor,
                    (0x1, 0x00) => Opcode::Sll,
                    (0x5, 0x00) => Opcode::Srl,
                    (0x5, 0x20) => Opcode::Sra,
                    _ => Opcode::Unknown,
                };
                Instruction { opcode, rd, rs1, rs2, imm: 0 }
            }
            0x13 => {
                // i-type alu
                let imm = sign_extend(raw >> 20, 12);
                let opcode = match funct3 {
                    0x0 => Opcode::Addi,
                    0x7 => Opcode::Andi,
                    0x6 => Opcode::Ori,
                    0x4 => Opcode::Xori,
                    0x1 => Opcode::Slli,
                    0x5 => {
                        if funct7 == 0x00 {
                            Opcode::Srli
                        } else if funct7 == 0x20 {
                            Opcode::Srai
                        } else {
                            Opcode::Unknown
                        }
                    }
                    _ => Opcode::Unknown,
                };
                Instruction { opcode, rd, rs1, rs2: 0, imm }
            }
            0x03 => {
                // load
                let imm = sign_extend(raw >> 20, 12);
                Instruction { opcode: Opcode::Lw, rd, rs1, rs2: 0, imm }
            }
            0x23 => {
                // store
                let imm_low = (raw >> 7) & 0x1f;
                let imm_high = (raw >> 25) & 0x7f;
                let imm = sign_extend((imm_high << 5) | imm_low, 12);
                Instruction { opcode: Opcode::Sw, rd: 0, rs1, rs2, imm }
            }
            0x63 => {
                // branch
                let imm_11 = (raw >> 7) & 0x1;
                let imm_4_1 = (raw >> 8) & 0xf;
                let imm_10_5 = (raw >> 25) & 0x3f;
                let imm_12 = (raw >> 31) & 0x1;
                let imm = (imm_12 << 12) | (imm_11 << 11) | (imm_10_5 << 5) | (imm_4_1 << 1);
                let imm = sign_extend(imm, 13);
                let opcode = match funct3 {
                    0x0 => Opcode::Beq,
                    0x1 => Opcode::Bne,
                    0x4 => Opcode::Blt,
                    0x5 => Opcode::Bge,
                    _ => Opcode::Unknown,
                };
                Instruction { opcode, rd: 0, rs1, rs2, imm }
            }
            0x37 => {
                // lui
                let imm = (raw & 0xfffff000) as i32;
                Instruction { opcode: Opcode::Lui, rd, rs1: 0, rs2: 0, imm }
            }
            0x17 => {
                // auipc
                let imm = (raw & 0xfffff000) as i32;
                Instruction { opcode: Opcode::Auipc, rd, rs1: 0, rs2: 0, imm }
            }
            0x6f => {
                // jal
                let imm_19_12 = (raw >> 12) & 0xff;
                let imm_11 = (raw >> 20) & 0x1;
                let imm_10_1 = (raw >> 21) & 0x3ff;
                let imm_20 = (raw >> 31) & 0x1;
                let imm = (imm_20 << 20) | (imm_19_12 << 12) | (imm_11 << 11) | (imm_10_1 << 1);
                let imm = sign_extend(imm, 21);
                Instruction { opcode: Opcode::Jal, rd, rs1: 0, rs2: 0, imm }
            }
            0x67 => {
                // jalr
                let imm = sign_extend(raw >> 20, 12);
                Instruction { opcode: Opcode::Jalr, rd, rs1, rs2: 0, imm }
            }
            _ => Instruction {
                opcode: Opcode::Unknown,
                rd: 0,
                rs1: 0,
                rs2: 0,
                imm: 0,
            },
        }
    }

    pub fn disassemble(&self) -> String {
        match self.opcode {
            Opcode::Add => format!("add x{}, x{}, x{}", self.rd, self.rs1, self.rs2),
            Opcode::Sub => format!("sub x{}, x{}, x{}", self.rd, self.rs1, self.rs2),
            Opcode::And => format!("and x{}, x{}, x{}", self.rd, self.rs1, self.rs2),
            Opcode::Or => format!("or x{}, x{}, x{}", self.rd, self.rs1, self.rs2),
            Opcode::Xor => format!("xor x{}, x{}, x{}", self.rd, self.rs1, self.rs2),
            Opcode::Sll => format!("sll x{}, x{}, x{}", self.rd, self.rs1, self.rs2),
            Opcode::Srl => format!("srl x{}, x{}, x{}", self.rd, self.rs1, self.rs2),
            Opcode::Sra => format!("sra x{}, x{}, x{}", self.rd, self.rs1, self.rs2),
            Opcode::Addi => format!("addi x{}, x{}, {}", self.rd, self.rs1, self.imm),
            Opcode::Andi => format!("andi x{}, x{}, {}", self.rd, self.rs1, self.imm),
            Opcode::Ori => format!("ori x{}, x{}, {}", self.rd, self.rs1, self.imm),
            Opcode::Xori => format!("xori x{}, x{}, {}", self.rd, self.rs1, self.imm),
            Opcode::Slli => format!("slli x{}, x{}, {}", self.rd, self.rs1, self.imm & 0x1f),
            Opcode::Srli => format!("srli x{}, x{}, {}", self.rd, self.rs1, self.imm & 0x1f),
            Opcode::Srai => format!("srai x{}, x{}, {}", self.rd, self.rs1, self.imm & 0x1f),
            Opcode::Lw => format!("lw x{}, {}(x{})", self.rd, self.imm, self.rs1),
            Opcode::Sw => format!("sw x{}, {}(x{})", self.rs2, self.imm, self.rs1),
            Opcode::Beq => format!("beq x{}, x{}, {}", self.rs1, self.rs2, self.imm),
            Opcode::Bne => format!("bne x{}, x{}, {}", self.rs1, self.rs2, self.imm),
            Opcode::Blt => format!("blt x{}, x{}, {}", self.rs1, self.rs2, self.imm),
            Opcode::Bge => format!("bge x{}, x{}, {}", self.rs1, self.rs2, self.imm),
            Opcode::Lui => format!("lui x{}, 0x{:x}", self.rd, (self.imm as u32) >> 12),
            Opcode::Auipc => format!("auipc x{}, 0x{:x}", self.rd, (self.imm as u32) >> 12),
            Opcode::Jal => format!("jal x{}, {}", self.rd, self.imm),
            Opcode::Jalr => format!("jalr x{}, {}(x{})", self.rd, self.imm, self.rs1),
            Opcode::Unknown => format!("unknown"),
        }
    }
}

fn sign_extend(val: u32, bits: u32) -> i32 {
    let shift = 32 - bits;
    ((val << shift) as i32) >> shift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_add() {
        // add x1, x2, x3
        let raw = 0x003100b3;
        let inst = Instruction::decode(raw);
        assert_eq!(inst.opcode, Opcode::Add);
        assert_eq!(inst.rd, 1);
        assert_eq!(inst.rs1, 2);
        assert_eq!(inst.rs2, 3);
    }

    #[test]
    fn test_decode_addi() {
        // addi x1, x2, 42
        let raw = 0x02a10093;
        let inst = Instruction::decode(raw);
        assert_eq!(inst.opcode, Opcode::Addi);
        assert_eq!(inst.rd, 1);
        assert_eq!(inst.rs1, 2);
        assert_eq!(inst.imm, 42);
    }

    #[test]
    fn test_sign_extend_negative() {
        let val = 0xfff; // -1 in 12-bit
        let extended = sign_extend(val, 12);
        assert_eq!(extended, -1);
    }
}
