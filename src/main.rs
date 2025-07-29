//! CLI entry point for the JS function scanner.

// Modules are now in lib.rs

use std::env;
use std::path::Path;
use cycode::types::FunctionInfo;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }
    let dir = Path::new(&args[1]);
    if !dir.is_dir() {
        eprintln!("Error: '{}' is not a directory", dir.display());
        std::process::exit(1);
    }
    let js_files = cycode::scanner::find_js_files(dir);
    let mut all_functions: Vec<FunctionInfo> = Vec::new();
    for file in js_files {
        let functions = cycode::parser::extract_functions(&file);
        all_functions.extend(functions);
    }
    println!("{}", serde_json::to_string_pretty(&all_functions).unwrap());
}
