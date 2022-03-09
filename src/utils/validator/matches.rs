use super::{ValidationError, ValidationErrorMap};

pub fn matches(errors: &mut ValidationErrorMap, field: &str, target: &str, matches: bool) {
    if !matches {
        errors.insert(
            field.to_string(),
            ValidationError::Match {
                field: field.to_string(),
                target: target.to_string(),
            },
        );
    }
}
