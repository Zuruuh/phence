use php_parser_rs::parser::ast::Program;
use puff_config::RulesConfig;
use serde::{de::value::MapDeserializer, Deserialize};

use crate::rule::{CodeSample, Rule, RuleDefinition};

const RULE_ID: &'static str = "quoting";

#[derive(Debug)]
pub struct QuotingStyleRule(QuotingStyleConfig);

impl TryFrom<&RulesConfig> for QuotingStyleRule {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &RulesConfig) -> Result<Self, Self::Error> {
        let config = value
            .get(RULE_ID)
            .cloned()
            .map(|config| QuotingStyleConfig::deserialize(MapDeserializer::new(config.into_iter())))
            .unwrap_or_else(|| Ok(QuotingStyleConfig::default()))
            .map_err(|err| Box::new(err))?;

        Ok(Self(config))
    }
}

impl Rule for QuotingStyleRule {
    fn definition(&self) -> RuleDefinition {
        RuleDefinition::new(
            "quoting",
            "Configure the type of quote used when representing string literals",
        )
        .with_code_sample(CodeSample::new(
            r#"<?php echo "Hello, World";"#,
            Some(QuotingStyleConfig {
                style: QuotingStyle::Single,
                ..Default::default()
            }),
        ))
    }

    fn applies(&self, _token: &Program) -> bool {
        todo!()
    }

    fn apply(&self, _token: Program) -> Program {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuotingStyleConfig {
    pub enabled: bool,
    pub style: QuotingStyle,
    pub ignore_escaped_quotes: bool,
    pub interpolate_vars_in_double_quotes: QuotingStyleVarInterpolation,
}

impl Default for QuotingStyleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            style: QuotingStyle::Single,
            ignore_escaped_quotes: false,
            interpolate_vars_in_double_quotes: QuotingStyleVarInterpolation::Preserve,
        }
    }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum QuotingStyle {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "double")]
    Double,
}

#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum QuotingStyleVarInterpolation {
    #[serde(rename = "preserve")]
    Preserve,
    #[serde(rename = "concat")]
    Concat,
    #[serde(rename = "never")]
    Never,
}

#[cfg(test)]
mod test {
    use crate::rules::quoting_style::{
        QuotingStyle, QuotingStyleConfig, QuotingStyleRule, QuotingStyleVarInterpolation,
    };

    #[test]
    pub fn test() {
        let config: Result<QuotingStyleConfig, _> = toml::from_str(
            r#"
            enabled = true
            style = "single"
            ignore_escaped_quotes = false
            interpolate_vars_in_double_quotes = "preserve"
        "#,
        );

        assert_eq!(
            &Ok(QuotingStyleConfig {
                enabled: true,
                style: QuotingStyle::Single,
                ignore_escaped_quotes: false,
                interpolate_vars_in_double_quotes: QuotingStyleVarInterpolation::Preserve,
            }),
            &config
        );
        let config = config.unwrap();

        let rule = QuotingStyleRule(config);

        dbg!(rule);
    }
}
