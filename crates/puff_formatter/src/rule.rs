use php_parser_rs::parser::ast::Program;

pub trait Rule {
    fn definition(&self) -> RuleDefinition;
    fn applies(&self, token: &Program) -> bool;
    fn apply(&self, token: Program) -> Program;
}

pub struct RuleDefinition {
    pub id: &'static str,
    pub summary: &'static str,
    pub code_samples: Vec<CodeSample>,
    pub risk_reason: Option<String>,
}

impl RuleDefinition {
    pub fn new(id: &'static str, summary: &'static str) -> Self {
        Self {
            id,
            summary,
            code_samples: Vec::new(),
            risk_reason: None,
        }
    }

    pub fn with_code_sample(mut self, sample: CodeSample) -> Self {
        self.code_samples.push(sample);

        self
    }

    pub fn with_risk_reason(mut self, reason: String) -> Self {
        self.risk_reason = Some(reason);

        self
    }
}

pub struct CodeSample {
    pub code: &'static str,
    pub config: Option<String>,
}

impl CodeSample {
    pub fn new(code: &'static str, config: Option<impl serde::Serialize>) -> Self {
        Self {
            code,
            config: config.map(|config| toml::to_string(&config).expect("TODO HANDLE THIS")),
        }
    }
}
