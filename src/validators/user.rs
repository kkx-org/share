use crate::models::user::CreateUserDto;
use crate::utils::validator::{length, matches, ValidationErrorMap, ValidationErrors, Validator};

impl Validator for CreateUserDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors: ValidationErrorMap = ValidationErrorMap::new();

        length(
            &mut errors,
            "username",
            self.username.len(),
            Some(2),
            Some(20),
        );
        length(
            &mut errors,
            "password",
            self.password.len(),
            Some(8),
            Some(128),
        );

        matches(
            &mut errors,
            "password",
            "confirm_password",
            self.password == self.confirm_password,
        );

        if errors.len() != 0 {
            Err(ValidationErrors(errors))
        } else {
            Ok(())
        }
    }
}
