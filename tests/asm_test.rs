use rv32_emu::assembler::Assembler;

#[test]
fn test_simple_program() {
    let mut asm = Assembler::new();
    let source = r#"
addi x1, x0, 10
addi x2, x0, 20
add x3, x1, x2
"#;
    
    let code = asm.assemble(source).unwrap();
    assert_eq!(code.len(), 12); // 3 instructions
}

#[test]
fn test_with_labels() {
    let mut asm = Assembler::new();
    let source = r#"
    addi x1, x0, 0
loop:
    addi x1, x1, 1
    beq x1, x2, end
    jal x0, loop
end:
    addi x3, x0, 42
"#;
    
    let result = asm.assemble(source);
    assert!(result.is_ok());
}

#[test]
fn test_comments() {
    let mut asm = Assembler::new();
    let source = r#"
# this is a comment
addi x1, x0, 10  # another comment
# more comments
add x2, x1, x1
"#;
    
    let code = asm.assemble(source).unwrap();
    assert_eq!(code.len(), 8); // 2 instructions
}

#[test]
fn test_memory_operations() {
    let mut asm = Assembler::new();
    let source = r#"
lui x1, 0x100
sw x2, 0(x1)
lw x3, 4(x1)
"#;
    
    let code = asm.assemble(source).unwrap();
    assert_eq!(code.len(), 12);
}
