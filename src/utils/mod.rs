use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VeronError {
    ExecutionError(String),
    CompilationError(String),
}

impl Error for VeronError {}

impl fmt::Display for VeronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VeronError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            VeronError::CompilationError(msg) => write!(f, "Compilation error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, VeronError>;