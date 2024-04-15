use rayon::prelude::*;
use std::{any::Any, sync::Arc};

use anyhow::Result;

use crate::{ValidationConfig, ValidationResult};

pub type RuleFunction = fn(
    input: Arc<dyn Any + Sync + Send>,
    config: Arc<ValidationConfig>,
) -> Result<ValidationResult>;

pub trait RuleBuilder {
    fn build_rules() -> Vec<RuleFunction>;
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub config: Arc<ValidationConfig>,
    pub rules: Vec<RuleFunction>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            config: Arc::new(ValidationConfig::new()),
            rules: vec![],
        }
    }

    pub fn with_config(mut self, config: &ValidationConfig) -> Self {
        self.config = Arc::new(config.to_owned());
        self
    }

    pub fn with_rule(&mut self, rule: RuleFunction) -> &mut Self {
        self.rules.push(rule);
        self
    }

    pub fn with_rules<T>(mut self) -> Self
    where
        T: RuleBuilder,
    {
        let mut new_rules = T::build_rules();
        self.rules.append(&mut new_rules);

        self
    }

    pub fn eval(&self, obj: Arc<dyn Any + Send + Sync>) -> Result<Vec<ValidationResult>> {
        let results: Result<Vec<ValidationResult>> = self
            .rules
            .par_iter()
            .map(|rule| rule(obj.clone(), self.config.clone()))
            .collect::<Vec<Result<ValidationResult>>>()
            .into_iter()
            .collect();
        results
    }
}
