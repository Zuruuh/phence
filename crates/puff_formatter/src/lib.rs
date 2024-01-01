pub mod rule;
pub mod rule_set;
mod rules;

use php_parser_rs::parser::ast::Program;
use puff_config::Config;

pub fn format_file(content: &Program, config: &Config) {
    println!("Hello, world!");
}
