//! AetherScript Compiler
//! Week 3 Implementation

use std::env;
use std::fs;

mod parser;
mod passes;
mod codegen;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: aetherc <input.aethersrc>");
        std::process::exit(1);
    }
    
    let source = fs::read_to_string(&args[1])
        .expect("Failed to read source file");
    
    // Week 3: Parse → Annotate → Generate
    println!("AetherScript Compiler v0.1.0");
    println!("Compiling: {}", args[1]);
    
    // TODO: Implement full pipeline
    println!("Output: (stub)");
}
