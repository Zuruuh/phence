mod recommended;

use puff_config::FormatterConfig;

use crate::rule::Rule;

pub trait RuleSet {
    fn get(
        &self,
        config: &FormatterConfig,
    ) -> Result<Vec<Box<dyn Rule>>, Box<dyn std::error::Error>>;
}
