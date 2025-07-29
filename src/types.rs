//! Data structures for representing JavaScript function information.

use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct FunctionInfo {
    /// Unique identifier for the function (file + name + position)
    pub id: String,
    /// Path to the file containing the function
    pub file_path: String,
    /// Name of the function (or None for anonymous)
    pub name: Option<String>,
    /// Start position (line, column)
    pub start: (usize, usize),
    /// End position (line, column)
    pub end: (usize, usize),
}
