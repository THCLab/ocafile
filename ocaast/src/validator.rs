use crate::{ast::{OCAAst, Command, CommandType, ObjectKind}, errors::Error};

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
                let version_validator = validate_1_0_0(ast, command);
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

fn validate_1_0_0(ast: &OCAAst, command: Command) -> Result<(bool), Error> {
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

/// Check rule for remove command
/// Rule would be valid if attributes which commands tries to remove exist in the stack
///
/// # Arguments
/// * `ast` - valid OCA AST
/// * `command` - Command to validate against AST
///
/// # Returns
/// * `Result<bool, Error>` - Result of validation
fn rule_remove_if_exist(ast: &OCAAst, command: Command) -> Result<bool, Error> {
    let mut valid = true;
    let mut errors = Vec::new();
    // Create a list of all attributes ADDed and REMOVEd via commands and check if what left covers needs of new command
    let mut attributes: Vec<String> = Vec::new();
    for command in &ast.commands {
        match command.kind {
            CommandType::Remove => {
                if command.object_kind == ObjectKind::CaptureBase {
                    let attrs = command.content.as_ref().unwrap().attributes.as_ref().unwrap();
                    attributes.retain(|key| !attrs.contains_key(key));
                }
            }
            CommandType::Add => {
                if command.object_kind == ObjectKind::CaptureBase {
                    let attrs = command.content.as_ref().unwrap().attributes.as_ref().unwrap();
                    attributes.extend(attrs.keys().cloned());
                }
            }
            _ => {}
        }
    }
    let keys_to_remove = command.content.as_ref().unwrap().attributes.as_ref().unwrap();
    valid = keys_to_remove.keys().all(|key| attributes.contains(key));

    if valid {
        Ok(true)
    } else {
        Err(Error::Validation(errors))
    }
}


#[cfg(test)]
mod tests {
    use indexmap::indexmap;

    use super::*;
    use crate::ast::{OCAAst, Command, CommandType, ObjectKind, Content, NestedValue};

    #[test]
    fn test_rule_remove_if_exist() {
        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some(Content {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedValue::Value("Text".to_string()),
                    "documentType".to_string() => NestedValue::Value("Text".to_string()),
                    "photo".to_string() => NestedValue::Value("Binary".to_string()),
                }),
                properties: None,
            }),
        };

        let command2 = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase,
            content: Some(Content {
                attributes: Some(indexmap! {
                    "issuer".to_string() => NestedValue::Value("Text".to_string()),
                    "last_name".to_string() => NestedValue::Value("Binary".to_string()),
                }),
                properties: None,
            }),
        };

        let remove_command = Command {
            kind: CommandType::Remove,
            object_kind: ObjectKind::CaptureBase,
            content: Some(Content {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedValue::Value("".to_string()),
                    "issuer".to_string() => NestedValue::Value("".to_string()),
                }),
                properties: None,
            }),
        };

        let remove_command2 = Command {
            kind: CommandType::Remove,
            object_kind: ObjectKind::CaptureBase,
            content: Some(Content {
                attributes: Some(indexmap! {
                    "name".to_string() => NestedValue::Value("".to_string()),
                    "photo".to_string() => NestedValue::Value("".to_string()),
                }),
                properties: None,
            }),
        };

        let mut ocaast = OCAAst::new();
        ocaast.commands.push(command);
        ocaast.commands.push(command2);
        let mut result = rule_remove_if_exist(&ocaast, remove_command.clone());
        assert!(result.is_ok());
        ocaast.commands.push(remove_command2);
        result = rule_remove_if_exist(&ocaast, remove_command);
        assert!(result.is_err());
    }
}