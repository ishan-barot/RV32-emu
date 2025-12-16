# compute fibonacci numbers
# result stored in x10

    addi x1, x0, 0      # fib(0) = 0
    addi x2, x0, 1      # fib(1) = 1
    addi x3, x0, 10     # n = 10 (compute fib(10))
    addi x4, x0, 0      # counter

loop:
    beq x4, x3, done
    add x5, x1, x2      # next = a + b
    addi x1, x2, 0      # a = b
    addi x2, x5, 0      # b = next
    addi x4, x4, 1      # counter++
    jal x0, loop

done:
    addi x10, x1, 0     # result in x10
    # normally would halt here but we just loop at 0
    jal x0, 0
