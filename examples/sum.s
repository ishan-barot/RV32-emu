# sum numbers from 1 to n
# result in x10

    addi x1, x0, 0      # sum = 0
    addi x2, x0, 1      # i = 1
    addi x3, x0, 100    # n = 100

loop:
    bge x2, x3, done    # if i >= n, done
    add x1, x1, x2      # sum += i
    addi x2, x2, 1      # i++
    jal x0, loop

done:
    addi x10, x1, 0     # result
    jal x0, 0
