use super::rules::{abstract_rule::BusinessRule, error::BusinessRuleError};

pub trait Entity {
    fn check_rule(rule: impl BusinessRule) -> Result<(), BusinessRuleError> {
        if rule.is_broken() {
            return Err(BusinessRuleError {
                message: rule.message(),
            });
        }
        Ok(())
    }
}
