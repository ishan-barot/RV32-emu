// core cpu state: registers, memory, pc

pub const NREGS: usize = 32;
pub const MEM_SIZE: usize = 1024 * 1024; // 1mb for now

pub struct Cpu {
    pub regs: [u32; NREGS],
    pub pc: u32,
    pub mem: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: [0; NREGS],
            pc: 0,
            mem: vec![0; MEM_SIZE],
        }
    }

    pub fn load_program(&mut self, data: &[u8], addr: u32) {
        let start = addr as usize;
        let end = start + data.len();
        if end > self.mem.len() {
            panic!("program too large");
        }
        self.mem[start..end].copy_from_slice(data);
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        // TODO: add misaligned access trap
        if addr + 4 > self.mem.len() {
            panic!("memory access out of bounds: 0x{:x}", addr);
        }
        u32::from_le_bytes([
            self.mem[addr],
            self.mem[addr + 1],
            self.mem[addr + 2],
            self.mem[addr + 3],
        ])
    }

    pub fn write_word(&mut self, addr: u32, val: u32) {
        let addr = addr as usize;
        if addr + 4 > self.mem.len() {
            panic!("memory write out of bounds: 0x{:x}", addr);
        }
        let bytes = val.to_le_bytes();
        self.mem[addr..addr + 4].copy_from_slice(&bytes);
    }

    pub fn write_reg(&mut self, rd: usize, val: u32) {
        if rd != 0 {
            self.regs[rd] = val;
        }
    }

    pub fn read_reg(&self, rs: usize) -> u32 {
        self.regs[rs]
    }

    pub fn reset(&mut self) {
        self.regs = [0; NREGS];
        self.pc = 0;
    }
}
