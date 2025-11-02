use clap::{Parser, ValueHint};
use mini_grep::AppError;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Config {
    /// Ignore_case sensitivity (flag), default = sensitive
    #[arg(short, long = "ignore-case")]
    ignore_case: bool,
    #[arg(value_name = "PATTERN")]
    /// Search pattern
    pattern: String,
    #[arg(short, long)]
    /// Number of lines that contains or equals search pattern
    count: bool,
    #[arg(short = 'v', long = "invert-match")]
    invert: bool,
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    /// File(s) to search or "-" for input search
    path: Vec<PathBuf>,
}
impl Config {
    pub fn try_parse() -> Result<Self, AppError> {
        match <Self as Parser>::try_parse() {
            Ok(config) => {
                if let Err(e) = config.validate() {
                    return Err(AppError::UnknownArguments(e.to_string()));
                }
                Ok(config)
            }
            Err(e) => {
                return Err(AppError::ConfigParseError(e.kind().to_string()));
            }
        }
    }
    fn validate(&self) -> Result<(), AppError> {
        if self.pattern.trim().is_empty() {
            return Err(AppError::PatternRequired);
        }
        for file in &self.path {
            if file.as_os_str() == "-" {
                continue;
            }
            for file in &self.path {
                if !file.is_file() {
                    return Err(AppError::InvalidFile(file.display().to_string()));
                }
            }
        }
        Ok(())
    }
}
