use std::error::Error;

use puff_config::FormatterConfig;

use crate::{rule::Rule, rules::quoting_style::QuotingStyleRule};

use super::RuleSet;

pub struct RecommendedRuleSet;

impl RuleSet for RecommendedRuleSet {
    fn get(&self, config: &FormatterConfig) -> Result<Vec<Box<dyn Rule>>, Box<dyn Error>> {
        Ok(vec![Box::new(QuotingStyleRule::try_from(&config.rules)?)])
    }
}
