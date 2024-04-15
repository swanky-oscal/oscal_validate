pub use config::*;
pub use documents::*;
pub use engine::*;
pub use error::Error;
pub use validation_result::*;

mod config;
mod documents;
mod engine;
mod error;
mod validation_result;

#[cfg(test)]
mod tests {
    use oscal_lib::{poam::PlanOfActionAndMilestones, Rulable};

    use super::*;
    use crate::Engine;

    #[test]
    fn test_rules() {
        //let poam = load_poam().expect("failed to load poam");
        let json = include_str!("../../../oscal/fedramp-automation/dist/content/rev5/templates/poam/json/FedRAMP-POAM-OSCAL-Template.json");
        let poam = PlanOfActionAndMilestones::parse_json(json).expect("oops");
        let config = ValidationConfig::new();
        let engine = Engine::new()
            .with_config(&config)
            .with_rules::<PlanOfActionAndMilestones>();
        let result = engine.eval(poam);
        dbg!(&result);
        assert!(result.is_ok());
    }
}
