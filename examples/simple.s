# simple test program

addi x1, x0, 42
addi x2, x1, 8
add x3, x1, x2
sub x4, x3, x1

# store to memory
lui x5, 0x10
sw x4, 0(x5)
lw x6, 0(x5)

# branch test
beq x4, x6, success
jal x0, 0

success:
    addi x10, x0, 1
    jal x0, 0
