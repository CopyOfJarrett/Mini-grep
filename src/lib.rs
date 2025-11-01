use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Config {
    /// IGNORE_CASE sensitivity (flag), default = sensitive
    #[arg(short, long = "ignore-case")]
    ignore_case: bool,
    #[arg(value_name = "PATTERN")]
    /// Search PATTERN
    pattern: String,
    #[arg(short, long)]
    /// Number of lines that contains or equals search PATTERN
    count: bool,
    #[arg(short = 'v', long = "invert-match")]
    invert: bool,
    #[arg(value_name = "FILE", value_hint = ValueHint::FilePath)]
    /// Path of FILE(s) to search
    path: Vec<PathBuf>,
}
impl Config {
    /// Unknown/missing flages : exit(3) Other: exit(2)
    pub fn try_parse() -> Self {
        match <Self as Parser>::try_parse() {
            Ok(config) => {
                if let Err(e) = config.validate() {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
                config
            }
            Err(e) => {
                let _ = e.print();
                std::process::exit(2);
            }
        }
    }
    fn validate(&self) -> Result<(), String> {
        for file in &self.path {
            if file.as_os_str() == "-" {
                continue;
            }
            for file in &self.path {
                if !file.is_file() {
                    return Err(format!("Invalid FILE: {}", file.display()));
                }
            }
        }
        Ok(())
    }
}
