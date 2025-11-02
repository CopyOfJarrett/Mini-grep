use clap::{Parser, ValueHint};
use mini_grep::{AppError, Arguments, CaseMode, MatchMode, OutputMode};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// Ignore_case sensitivity (flag), default = sensitive
    #[arg(short, long = "Ignore-case")]
    pub ignore_case: bool,
    #[arg(value_name = "PATTERN")]
    /// Search pattern
    pub pattern: String,
    #[arg(short, long)]
    /// Number of lines that contains or equals search pattern
    pub count: bool,
    #[arg(short = 'v', long = "invert-match")]
    pub invert: bool,
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    /// File(s) to search or "-" for input search
    pub path: Vec<PathBuf>,
}
impl Config {
    pub fn try_parse() -> Result<Self, AppError> {
        match <Self as Parser>::try_parse() {
            Ok(config) => {
                config.validate()?;
                Ok(config)
            }
            Err(e) => {
                return Err(AppError::ConfigParseError(e.to_string()));
            }
        }
    }
    fn validate(&self) -> Result<(), AppError> {
        if self.pattern.trim().is_empty() {
            return Err(AppError::PatternRequired);
        }
        for p in &self.path {
            if p == Path::new("-") || p.as_os_str() == OsStr::new("-") {
                continue;
            }
            if !p.is_file() {
                return Err(AppError::InvalidFile(p.display().to_string()));
            }
        }
        Ok(())
    }
    pub fn get_arguments(&self) -> Arguments {
        Arguments {
            case: if self.ignore_case {
                CaseMode::Insensitive
            } else {
                CaseMode::Sensitive
            },
            invert: if self.invert {
                MatchMode::Invert
            } else {
                MatchMode::Default
            },
            count: if self.count {
                OutputMode::Count
            } else {
                OutputMode::Default
            },
        }
    }
    pub fn get_pattern(&self) -> &str {
        self.pattern.as_str()
    }
    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.path
    }
}
