# Lubalia
A small programming language I made for embedding in my projects, but you can use it too (mentioning me somewhere).

> I'm working on making it compatible with JS throught WASM

## Basic syntax
```
let name = "César";

@print("Hello, $x!");
```

## How does it work
- Compiler: Converts lubacode into bytecode for the LVM (Lubalia Virtual Machine)
- Interpreter: Executes the bytecode in the LVM