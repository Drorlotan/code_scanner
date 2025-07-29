//! Unit tests for scanner.rs

use std::fs::{self, File};
use std::path::Path;
use cycode::scanner::find_js_files;

#[test]
fn test_find_js_files() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let js_path = tmp_dir.path().join("test.js");
    let _ = File::create(&js_path).unwrap();
    let txt_path = tmp_dir.path().join("test.txt");
    let _ = File::create(&txt_path).unwrap();
    let files = find_js_files(tmp_dir.path());
    assert!(files.contains(&js_path));
    assert!(!files.contains(&txt_path));
}
