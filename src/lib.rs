pub mod errors;
pub use errors::AppError;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum CaseMode {
    Sensitive,
    Insensitive,
}
#[derive(Debug, Clone, Copy)]
pub enum MatchMode {
    Default,
    Invert,
}
#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    Default,
    Count,
}
#[derive(Debug, Clone, Copy)]
pub struct Arguments {
    pub case: CaseMode,
    pub invert: MatchMode,
    pub count: OutputMode,
}
#[derive(Debug, Clone)]
pub struct Query {
    needle: String,
    options: Arguments,
    path: Vec<PathBuf>,
}
impl Query {
    pub fn new(pattern: &str, arguments: Arguments, path: &[PathBuf]) -> Self {
        let needle = match arguments.case {
            CaseMode::Sensitive => pattern.to_string(),
            CaseMode::Insensitive => pattern.to_lowercase().to_string(),
        };
        Self {
            needle,
            options: arguments,
            path: path.to_vec(),
        }
    }
    pub fn match_lines(&self, line: &str) -> bool {
        line.contains(&self.needle)
    }
    pub fn options(&self) -> Arguments {
        self.options
    }
    pub fn needle(&self) -> &str {
        &self.needle
    }
    pub fn path(&self) -> &Vec<PathBuf> {
        &self.path
    }
}
pub fn default_search<R: BufRead>(
    query: &Query,
    options: Arguments,
    reader: R,
) -> Result<Vec<String>, AppError> {
    let invert: bool = match options.invert {
        MatchMode::Default => false,
        MatchMode::Invert => true,
    };
    let mut out: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let m = query.match_lines(&line);
        if invert ^ m {
            out.push(line);
        }
    }
    Ok(out)
}
pub fn count_search<R: BufRead>(
    query: &Query,
    options: Arguments,
    reader: R,
) -> Result<usize, AppError> {
    let invert: bool = match options.invert {
        MatchMode::Default => false,
        MatchMode::Invert => true,
    };
    let mut count: usize = 0;
    for line in reader.lines() {
        let line = line?;
        let m = query.match_lines(&line);
        if invert ^ m {
            count += 1;
        }
    }
    Ok(count)
}
pub fn run(query: &Query, options: Arguments, files: &[PathBuf]) -> Result<(), AppError> {
    if files.is_empty() {
        let stdin = io::stdin();
        let reader = stdin.lock();
        match options.count {
            OutputMode::Default => {
                for line in default_search(query, options, reader)? {
                    println!("{line}");
                }
            }
            OutputMode::Count => {
                let n = count_search(query, options, reader)?;
                println!("{n}");
            }
        }
        return Ok(());
    }

    match options.count {
        OutputMode::Default => {
            for path in files {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                for line in default_search(query, options, reader)? {
                    println!("{line}");
                }
            }
        }
        OutputMode::Count => {
            let mut total = 0usize;
            for path in files {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                total += count_search(query, options, reader)?;
            }
            println!("{total}");
        }
    }

    Ok(())
}
