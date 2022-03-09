mod length;
mod matches;

pub use length::length;
pub use matches::matches;

use serde::Serialize;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct ValidationErrors(pub ValidationErrorMap);

pub type ValidationErrorMap = HashMap<String, ValidationError>;

#[derive(Debug, Serialize)]
#[serde(tag = "code")]
pub enum ValidationError {
    #[serde(rename = "invalid_length")]
    Length {
        #[serde(skip)]
        field: String,
        min: Option<usize>,
        max: Option<usize>,
    },
    #[serde(rename = "invalid_email")]
    Email {
        #[serde(skip)]
        field: String,
    },
    #[serde(rename = "does_not_match")]
    Match {
        #[serde(skip)]
        field: String,
        target: String,
    }, // Nested {
       //     field: String,
       //     errors: ValidationErrorMap,
       // }
}

impl Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "validation failed: {:?}", self)
    }
}
impl std::error::Error for ValidationErrors {}

pub trait Validator {
    fn validate(&self) -> Result<(), ValidationErrors>;
}
