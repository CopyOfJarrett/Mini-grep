mod config;
use config::Config;
use mini_grep::{Query, run};

fn main() {
    let config = Config::try_parse().unwrap();
    let query = Query::new(&config.pattern, config.get_arguments(), &config.get_paths());
    let _ = run(&query, query.options(), query.path());
}
