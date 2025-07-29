//! Unit tests for parser.rs

use std::path::Path;
use cycode::parser::extract_functions;

#[test]
fn test_extract_named_function() {
    let js_code = "function foo() { return 42; }";
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), js_code).unwrap();
    let functions = extract_functions(tmp.path());
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name.as_deref(), Some("foo"));
}

#[test]
fn test_extract_anonymous_function() {
    let js_code = "const x = function() { return 1; };";
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), js_code).unwrap();
    let functions = extract_functions(tmp.path());
    assert!(functions.iter().any(|f| f.name.is_none()));
}

#[test]
fn test_extract_arrow_function() {
    let js_code = "const y = () => 2;";
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), js_code).unwrap();
    let functions = extract_functions(tmp.path());
    assert!(functions.iter().any(|f| f.name.is_none()));
}
