# rv32-emu

a partial risc-v rv32i emulator in rust with assembler, debugger, and performance metrics.

## quick start

```bash
cargo run -- run -f examples/fibonacci.s -p
```

## what's supported

the emulator implements a useful subset of rv32i:

**arithmetic/logic:** add, sub, and, or, xor, sll, srl, sra, addi, andi, ori, xori, slli, srli, srai  
**memory:** lw, sw (word-aligned only)  
**control flow:** beq, bne, blt, bge, jal, jalr  
**upper immediate:** lui, auipc

## what's not supported (yet)

- byte/halfword loads and stores (lb, lh, lbu, lhu, sb, sh)
- comparison instructions (slt, slti, sltu, sltiu)
- unsigned branches (bltu, bgeu)
- system instructions and csr access
- misaligned memory access traps
- proper elf32 loading (currently just loads raw binary)

the compatibility contract: for instructions that are supported, behavior matches the risc-v spec. unsupported instructions return an error.

## why these tradeoffs

- **word-only memory:** keeps the memory interface simple and fast. most real programs can work with word access + shifting. byte/halfword support is planned but adds complexity to the memory path.

- **no elf loader yet:** parsing elf32 is fiddly and i wanted to get the core emulator working first. there's a TODO for this but flat binaries work fine for testing.

- **simple halt detection:** currently detects halt by jumping to address 0. this is hacky but works for test programs. a proper ecall-based halt would be cleaner.

- **inline decoder in executor:** could have separated these more cleanly but the tight coupling actually helped during debugging. might refactor later if it becomes unwieldy.

## usage

### run a program

```bash
# run assembly file
cargo run -- run -f examples/fibonacci.s -p

# run with custom load address
cargo run -- run -f program.bin -a 0x1000 -p
```

### assemble only

```bash
cargo run -- asm -i examples/sum.s -o sum.bin
```

### interactive debugger

```bash
cargo run -- debug -f examples/fibonacci.s
```

debugger commands:
- `step` / `s` - execute one instruction
- `continue` / `c` - run until breakpoint
- `break <addr>` / `b` - set breakpoint
- `regs` / `r` - dump registers
- `mem <addr>` / `m` - inspect memory
- `dis [addr]` / `d` - disassemble

## performance metrics

with `-p` flag, the emulator tracks:
- total instructions executed
- mips (millions of instructions per second)
- branch statistics (taken vs not taken)
- instruction mix breakdown

example output:
```
executed 55 instructions

performance metrics:
  instructions executed: 55
  mips: 12.45

branch statistics:
  taken: 8 (72.7%)
  not taken: 3 (27.3%)

instruction mix:
  Addi         15 (27.3%)
  Beq          11 (20.0%)
  Add          10 (18.2%)
  ...
```

## examples

### fibonacci

computes fib(10) iteratively:

```bash
cargo run -- run -f examples/fibonacci.s -p
```

result ends up in register x10.

### sum 1 to n

sums integers from 1 to 100:

```bash
cargo run -- run -f examples/sum.s -p
```

## testing

```bash
cargo test
```

tests cover:
- instruction-level behavior for each opcode
- edge cases: overflow, underflow, sign extension
- branch offset calculations including backward branches
- x0 hardwired to zero
- shift amount masking
- memory isolation

## next steps

the obvious next milestone is rv32m (multiply/divide extension). after that, probably byte/halfword memory access and proper trap handling.

i'm also not sure about the current halt detection mechanism. it works but feels wrong. might add a simple ecall handler for proper program termination.

## building

requires rust 1.70+

```bash
cargo build --release
```

## license

mit
