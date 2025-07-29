//! Uses Tree-sitter to parse JavaScript files and extract function declarations.

use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Node};
use tree_sitter_javascript as javascript;
use crate::types::FunctionInfo;

/// Extracts all function declarations from a JavaScript file.
pub fn extract_functions(file_path: &Path) -> Vec<FunctionInfo> {
    let source = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let mut parser = Parser::new();
    parser.set_language(javascript::language()).expect("Error loading JS grammar");
    let tree = match parser.parse(&source, None) {
        Some(t) => t,
        None => return vec![],
    };
    let mut functions = Vec::new();
    let root_node = tree.root_node();
    visit_node(&source, file_path, root_node, &mut functions);
    functions
}

fn visit_node(source: &str, file_path: &Path, node: Node, functions: &mut Vec<FunctionInfo>) {
    // Include function_declaration and method_definition nodes
    if node.kind() == "function_declaration" || node.kind() == "method_definition" {
        let name = extract_name(node, source);
        let start = node.start_position();
        let end = node.end_position();
        let id = format!(
            "{}:{}:{}-{}:{}-{}",
            file_path.display(),
            name.clone().unwrap_or("anonymous".to_string()),
            start.row,
            start.column,
            end.row,
            end.column
        );
        functions.push(FunctionInfo {
            id,
            file_path: file_path.display().to_string(),
            name,
            start: (start.row + 1, start.column + 1),
            end: (end.row + 1, end.column + 1),
        });
    }
    for i in 0..node.child_count() {
        visit_node(source, file_path, node.child(i).unwrap(), functions);
    }
}

fn extract_name(node: Node, source: &str) -> Option<String> {
    match node.kind() {
        "function_declaration" => {
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "identifier" {
                    return Some(child.utf8_text(source.as_bytes()).unwrap_or("").to_string());
                }
            }
            None
        }
        "method_definition" => {
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "property_identifier" || child.kind() == "identifier" {
                    return Some(child.utf8_text(source.as_bytes()).unwrap_or("").to_string());
                }
            }
            None
        }
        "function_expression" | "arrow_function" => {
            // Try to get variable name from parent
            if let Some(parent) = node.parent() {
                if parent.kind() == "variable_declarator" {
                    for i in 0..parent.child_count() {
                        let child = parent.child(i).unwrap();
                        if child.kind() == "identifier" {
                            return Some(child.utf8_text(source.as_bytes()).unwrap_or("").to_string());
                        }
                    }
                }
                // Assignment expressions (e.g., obj.foo = function() {})
                if parent.kind() == "assignment_expression" {
                    let left = parent.child(0).unwrap();
                    if left.kind() == "identifier" || left.kind() == "property_identifier" {
                        return Some(left.utf8_text(source.as_bytes()).unwrap_or("").to_string());
                    }
                }
            }
            None
        }
        _ => None,
    }
}
