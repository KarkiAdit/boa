mod parser;
mod compiler;
mod interpreter;
mod lib;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use parser::parse;
use compiler::{compile_to_instrs, instr_to_str};
use interpreter::eval;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.snek> <output.s> [--interpret]", args[0]);
        std::process::exit(1);
    }

    let in_name = &args[1];
    let contents = std::fs::read_to_string(in_name).expect("Failed to read input file");
    let expr = parse(&contents);

    // If --interpret flag is passed, just evaluate the result and print
    if args.len() > 3 && args[3] == "--interpret" {
        let result = eval(&expr);
        println!("{}", result);
        return Ok(());
    }

    let out_name = &args[2];
    let instrs = compile_to_instrs(&expr);
    let result = instrs
        .iter()
        .map(instr_to_str)
        .collect::<Vec<_>>()
        .join("\n  ");

    let asm_program = format!(
        "
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
