use super::{ValidationError, ValidationErrorMap};

pub fn length(
    errors: &mut ValidationErrorMap,
    field: &str,
    len: usize,
    min: Option<usize>,
    max: Option<usize>,
) {
    let is_valid = match (min, max) {
        (Some(min), None) => len >= min,
        (None, Some(max)) => len <= max,
        (Some(min), Some(max)) => (min..max).contains(&len),
        _ => true,
    };

    if !is_valid {
        errors.insert(
            field.to_string(),
            ValidationError::Length {
                field: field.to_string(),
                min,
                max,
            },
        );
    }
}
