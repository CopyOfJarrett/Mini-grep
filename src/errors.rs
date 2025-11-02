use std::process::{ExitCode, Termination};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    ConfigParseError(String),
    #[error("Search pattern is required")]
    PatternRequired,
    #[error("Invalid search pattern error: {0}")]
    InvalidPattern(String),
    #[error("Unknown argument(s): {0}")]
    UnknownArguments(String),
    #[error("Invalid file: {0}")]
    InvalidFile(String),
    #[error("Error opening file")]
    ErrorOpeningFile,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
impl Termination for AppError {
    fn report(self) -> ExitCode {
        eprintln!("{self}");
        match self {
            AppError::ConfigParseError(_) => ExitCode::from(3),
            AppError::PatternRequired => ExitCode::from(3),
            AppError::InvalidPattern(_) => ExitCode::from(2),
            AppError::UnknownArguments(_) => ExitCode::from(2),
            AppError::InvalidFile(_) => ExitCode::from(2),
            AppError::ErrorOpeningFile => ExitCode::from(2),
            AppError::Io(_) => ExitCode::from(3),
        }
    }
}
