//! AetherScript Compiler v1.0
//! Complete implementation with parser, passes, and codegen

mod parser;
mod passes;
mod codegen;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("AetherScript Compiler v1.0");
        eprintln!("Usage: aetherc <input.aethersrc> [--output <file>]");
        eprintln!("\nOptions:");
        eprintln!("  --output <file>    Output file (default: output.rs)");
        eprintln!("  --verbose          Show compilation details");
        std::process::exit(1);
    }
    
    let input_file = &args[1];
    let mut output_file = "output.rs".to_string();
    let mut verbose = false;
    
    // Parse arguments
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                if i + 1 < args.len() {
                    output_file = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: --output requires a filename");
                    std::process::exit(1);
                }
            }
            "--verbose" => {
                verbose = true;
                i += 1;
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }
    }
    
    // Read source file
    let source = match fs::read_to_string(input_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", input_file, e);
            std::process::exit(1);
        }
    };
    
    if verbose {
        println!("Compiling: {}", input_file);
        println!("Source size: {} bytes", source.len());
    }
    
    // Parse
    if verbose {
        println!("\n[1/4] Parsing...");
    }
    let mut ast = match parser::parse(&source) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };
    
    if verbose {
        println!("  Found {} app(s)", ast.apps.len());
    }
    
    // Pass 1: Memory Annotation
    if verbose {
        println!("\n[2/4] Analyzing memory usage...");
    }
    passes::annotate_memory(&mut ast);
    
    // Pass 2: Task Partitioning
    if verbose {
        println!("\n[3/4] Partitioning tasks...");
    }
    passes::partition_tasks(&mut ast);
    
    // Pass 3: Optimization
    passes::optimize(&mut ast);
    
    // Code Generation
    if verbose {
        println!("\n[4/4] Generating Rust code...");
    }
    let rust_code = codegen::generate_rust(&ast);
    
    // Write output
    match fs::write(&output_file, &rust_code) {
        Ok(_) => {
            println!("✓ Compilation successful!");
            println!("  Output: {}", output_file);
            println!("  Size: {} bytes", rust_code.len());
        }
        Err(e) => {
            eprintln!("Error writing output: {}", e);
            std::process::exit(1);
        }
    }
}
