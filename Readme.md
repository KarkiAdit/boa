# 🐍 Boa: The Add Compiler for Snek Expressions

## 📌 Overview
Boa is a toy compiler written in Rust that translates a small language called **Snek** into x86-64 assembly. It supports basic expressions including `add1`, `sub1`, arithmetic operations, and `let` bindings with lexical scoping.

You can either:
- **Compile** Snek code to assembly and execute it, or
- **Interpret** expressions directly in memory for debugging.

## ⚙️ Usage

### 🔧 Compile & Run a `.snek` File

```bash
make test/add.run
./test/add.run
