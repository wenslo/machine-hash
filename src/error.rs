use thiserror::Error;

#[derive(Error, Debug)]
pub enum HardwareError {
    #[error("Failed to execute command: {0}")]
    CommandError(String),
    
    #[error("Failed to read file: {0}")]
    FileError(String),
    
    #[error("Failed to parse output: {0}")]
    ParseError(String),
    
    #[error("System not supported: {0}")]
    UnsupportedSystem(String),
} 