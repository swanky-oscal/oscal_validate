use anyhow::Result;
use std::{any::Any, sync::Arc};

use oscal_lib::{poam::PlanOfActionAndMilestones, Rulable};

use crate::{RuleBuilder, RuleFunction, ValidationConfig, ValidationResult};

pub fn rule_1(
    any: Arc<dyn Any + Sync + Send>,
    _config: Arc<ValidationConfig>,
) -> Result<ValidationResult> {
    let _poam = PlanOfActionAndMilestones::from_arc_any(any)?;
    println!("Rule 1 for POAM!");

    Ok(ValidationResult::ok())
}

pub fn rule_2(
    any: Arc<dyn Any + Sync + Send>,
    _config: Arc<ValidationConfig>,
) -> Result<ValidationResult> {
    let _poam = PlanOfActionAndMilestones::from_arc_any(any)?;
    println!("Rule 2 for POAM!");

    Ok(ValidationResult::ok())
}

impl RuleBuilder for PlanOfActionAndMilestones {
    fn build_rules() -> Vec<RuleFunction> {
        vec![rule_1, rule_2]
    }
}
