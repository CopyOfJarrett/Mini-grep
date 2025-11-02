pub mod errors;
pub use errors::AppError;

struct Options {
    ignore_case: bool,
    invert: bool,
    count: bool,
}
pub struct Query;
impl Query {
    pub fn compile_pattern(pattern: &str, ignore_case: bool) -> String {
        if ignore_case {
            return pattern.to_lowercase();
        }
        pattern.to_string()
    }
}
