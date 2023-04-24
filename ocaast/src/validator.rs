use crate::{ast::{OCAAst, Command}, errors::Error};

/// Validates given commands against existing valid OCA AST
///
/// # Arguments
/// * `ast` - valid OCA AST
/// * `command` - Command to validate against AST
///
/// # Returns
/// * `Result<bool, Error>` - Result of validation
pub trait Validator {
    fn validate(&self, ast: &OCAAst, command: Command) -> Result<bool, Error>;
}

pub struct OCAValidator {}

impl Validator for OCAValidator {
    fn validate(&self, ast: &OCAAst, command: Command) -> Result<bool, Error> {
        let mut errors = Vec::new();
        let mut valid = true;
        match ast.version.as_str() {
            "1.0.0" => {
                let version_validator = validate_1_0_0(ast);
                if version_validator.is_err() {
                    valid = false;
                    errors.push(version_validator.err().unwrap());
                }
            }
            "" => {
                valid = false;
                errors.push(Error::MissingVersion());
            }
            _ => {
                valid = false;
                errors.push(Error::InvalidVersion(ast.version.to_string()));
            }
        }
        if valid {
            Ok(true)
        } else {
            Err(Error::Validation(errors))
        }
    }
}

fn validate_1_0_0(ast: &OCAAst) -> Result<(bool), Error> {
    // Rules
    // Cannot remove if does not exist on stack
    // Cannot modify if does not exist on stack
    // Cannot add if already exists on stack
    // Attributes must have valid type
    let mut valid = true;
    let mut errors = Vec::new();

    for command in &ast.commands {

    }
    if valid {
        Ok(true)
    } else {
        Err(Error::Validation(errors))
    }
}

