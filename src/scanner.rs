//! Scans directories recursively for JavaScript files.

use std::fs;
use std::path::{Path, PathBuf};

/// Recursively find all .js files in a directory.
pub fn find_js_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if dir.is_dir() {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return files,
        };
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(find_js_files(&path));
                } else if let Some(ext) = path.extension() {
                    if ext == "js" {
                        files.push(path);
                    }
                }
            }
        }
    }
    files
}
