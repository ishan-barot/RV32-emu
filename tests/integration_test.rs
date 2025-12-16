use rv32_emu::*;

// basic instruction tests

#[test]
fn test_add_basic() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 10;
    cpu.regs[2] = 20;
    
    // add x3, x1, x2
    let inst = 0x002081b3u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[3], 30);
}

#[test]
fn test_add_overflow() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 0xffffffff;
    cpu.regs[2] = 1;
    
    // add x3, x1, x2
    let inst = 0x002081b3u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[3], 0); // wrapping behavior
}

#[test]
fn test_sub_basic() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 30;
    cpu.regs[2] = 10;
    
    // sub x3, x1, x2
    let inst = 0x402081b3u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[3], 20);
}

#[test]
fn test_sub_underflow() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 0;
    cpu.regs[2] = 1;
    
    // sub x3, x1, x2
    let inst = 0x402081b3u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[3], 0xffffffff); // wrapping
}

#[test]
fn test_addi_negative() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 10;
    
    // addi x2, x1, -5
    let inst = 0xffb08113u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[2], 5);
}

#[test]
fn test_lw_sw() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 0x100;
    cpu.regs[2] = 0xdeadbeef;
    
    // sw x2, 0(x1)
    let inst_sw = 0x0020a023u32;
    cpu.write_word(0, inst_sw);
    exec.step(&mut cpu, &mut metrics).unwrap();
    
    cpu.pc = 4;
    cpu.regs[3] = 0;
    
    // lw x3, 0(x1)
    let inst_lw = 0x0000a183u32;
    cpu.write_word(4, inst_lw);
    exec.step(&mut cpu, &mut metrics).unwrap();
    
    assert_eq!(cpu.regs[3], 0xdeadbeef);
}

#[test]
fn test_beq_taken() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 42;
    cpu.regs[2] = 42;
    
    // beq x1, x2, 8 (skip 2 instructions)
    let inst = 0x00208463u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.pc, 8);
}

#[test]
fn test_beq_not_taken() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 42;
    cpu.regs[2] = 43;
    
    // beq x1, x2, 8
    let inst = 0x00208463u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.pc, 4);
}

#[test]
fn test_blt_signed() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = (-5i32) as u32;
    cpu.regs[2] = 5;
    
    // blt x1, x2, 8
    let inst = 0x0020c463u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.pc, 8); // -5 < 5
}

#[test]
fn test_jal() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    // jal x1, 16
    let inst = 0x010000efu32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[1], 4); // return address
    assert_eq!(cpu.pc, 16); // jumped
}

#[test]
fn test_jalr() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[2] = 0x100;
    
    // jalr x1, 8(x2)
    let inst = 0x008100e7u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[1], 4);
    assert_eq!(cpu.pc, 0x108);
}

#[test]
fn test_lui_auipc() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    // lui x1, 0x12345
    let inst = 0x123450b7u32;
    cpu.write_word(0, inst);
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[1], 0x12345000);
    
    cpu.pc = 4;
    // auipc x2, 0x100
    let inst = 0x00100117u32;
    cpu.write_word(4, inst);
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[2], 0x100004);
}

#[test]
fn test_shift_operations() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 0xff;
    
    // slli x2, x1, 4
    let inst = 0x00409113u32;
    cpu.write_word(0, inst);
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[2], 0xff0);
    
    cpu.pc = 4;
    cpu.regs[3] = 0xff00;
    
    // srli x4, x3, 4
    let inst = 0x0041d213u32;
    cpu.write_word(4, inst);
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[4], 0x0ff0);
    
    cpu.pc = 8;
    cpu.regs[5] = 0x80000000u32;
    
    // srai x6, x5, 4
    let inst = 0x4042d313u32;
    cpu.write_word(8, inst);
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[6], 0xf8000000); // sign extend
}

// edge case tests

#[test]
fn test_x0_always_zero() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 42;
    
    // add x0, x1, x1 (should not modify x0)
    let inst = 0x00108033u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[0], 0);
}

#[test]
fn test_branch_backward() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.pc = 16;
    cpu.regs[1] = 10;
    cpu.regs[2] = 10;
    
    // beq x1, x2, -8 (backwards)
    let inst = 0xfe208ce3u32;
    cpu.write_word(16, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.pc, 8);
}

#[test]
fn test_branch_offset_alignment() {
    // paranoia: branch offset should be even (bit 0 always 0)
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 5;
    cpu.regs[2] = 5;
    
    // beq with offset 4
    let inst = 0x00208263u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.pc & 1, 0); // ensure aligned
}

#[test]
fn test_shift_amount_masking() {
    let mut cpu = cpu::Cpu::new();
    let mut exec = executor::Executor::new();
    let mut metrics = metrics::Metrics::new();
    
    cpu.regs[1] = 0xff;
    cpu.regs[2] = 36; // larger than 31
    
    // sll x3, x1, x2 (should only use lower 5 bits = 4)
    let inst = 0x002091b3u32;
    cpu.write_word(0, inst);
    
    exec.step(&mut cpu, &mut metrics).unwrap();
    assert_eq!(cpu.regs[3], 0xff << 4);
}

// this test is kind of paranoid but checks that we handle
// potential memory aliasing issues correctly
#[test]
fn test_memory_isolation() {
    let mut cpu = cpu::Cpu::new();
    
    cpu.write_word(0x100, 0xdeadbeef);
    cpu.write_word(0x104, 0xcafebabe);
    
    assert_eq!(cpu.read_word(0x100), 0xdeadbeef);
    assert_eq!(cpu.read_word(0x104), 0xcafebabe);
    
    // overwrite first
    cpu.write_word(0x100, 0x12345678);
    assert_eq!(cpu.read_word(0x100), 0x12345678);
    assert_eq!(cpu.read_word(0x104), 0xcafebabe); // should not change
}

// TODO: test misaligned memory access trap (not yet implemented)
// TODO: test instruction fetch from invalid address
