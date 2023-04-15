use crate::ocafile::{error::Error, Pair, Rule};
use log::{debug, info};
use oca_rs::state::{attribute::AttributeType, oca::overlay::Overlay};
use ocaast::{
    CaptureBaseContent, Command, CommandType, NestedValue, ObjectContent, ObjectKind,
    OverlayContent,
};
use std::{collections::HashMap, str::FromStr};

pub struct AddInstruction {}

impl AddInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, Error> {
        // let mut nested_object = None;
        let mut object_kind = None;
        let kind = CommandType::Add;
        let mut content = None;

        debug!("{}", record);
        for object in record.into_inner() {
            match object.as_rule() {
                Rule::meta => {
                    let mut properties: HashMap<String, NestedValue> = HashMap::new();
                    object_kind = Some(ObjectKind::Overlay(ocaast::OverlayType::Meta));
                    for attrs in object.into_inner() {
                        match attrs.as_rule() {
                            Rule::key_pairs => {
                                println!("Meta attr ----> {:?}", attrs);
                                properties.insert(
                                    "key".to_string(),
                                    NestedValue::Value(attrs.as_str().to_string()),
                                );
                            }
                            Rule::lang => {
                                debug!("Parsing language: {:?}", attrs.as_rule());
                                properties.insert(
                                    "lang".to_string(),
                                    NestedValue::Value(attrs.as_str().to_string()),
                                );
                            }
                            _ => {
                                return Err(Error::UnexpectedToken(format!(
                                    "Invalid attribute in meta overlay {:?}",
                                    attrs.as_rule()
                                )))
                            }
                        }
                    }
                }
                Rule::attribute => {
                    object_kind = Some(ObjectKind::CaptureBase);
                    let mut attributes: HashMap<String, NestedValue> = HashMap::new();
                    for attr_pairs in object.into_inner() {
                        match attr_pairs.as_rule() {
                            Rule::attr_pairs => {
                                info!("attribute: {:?}", attr_pairs);
                                for attr in attr_pairs.into_inner() {
                                    debug!("Parsing attribute {:?}", attr);
                                    if let Some((key, value)) =
                                        AddInstruction::extract_attribute(attr)
                                    {
                                        // TODO find out how to parse nested objects
                                        attributes.insert(key, NestedValue::Value(value));
                                    } else {
                                        debug!("Skipping attribute");
                                    }
                                }
                            }
                            _ => {
                                return Err(Error::UnexpectedToken(format!(
                                    "Invalid attributes in ATTRIBUTE instruction {:?}",
                                    attr_pairs.as_rule()
                                )))
                            }
                        }
                    }
                    content = Some(CaptureBaseContent {
                        properties: None,
                        attributes: Some(attributes),
                    });
                }
                Rule::comment => continue,
                // Rule::classification => continue,
                _ => {
                    return Err(Error::UnexpectedToken(format!(
                        "Overlay: unexpected token {:?}",
                        object.as_rule()
                    )))
                }
            };
        }

        // let instruction = Instruction {
        //     command: Command::Add,
        //     data: object,
        // };

        Ok(Command {
            kind: kind,
            object_kind: object_kind.unwrap(),
            content: Some(ObjectContent::CaptureBase(content.unwrap())),
        })
    }

    fn extract_attribute(attr_pair: Pair) -> Option<(String, String)> {
        let mut key = String::new();
        let mut value = String::new();

        for item in attr_pair.into_inner() {
            match item.as_rule() {
                Rule::key => {
                    key = item.as_str().to_string();
                }
                Rule::attr_type => match AttributeType::from_str(&item.as_span().as_str()) {
                    Ok(attr_type) => {
                        debug!("Attribute type: {:?}", attr_type);
                        value = attr_type.to_string();
                    }
                    Err(e) => {
                        panic!("Invalid attribute type {:?}", e);
                    }
                },
                _ => {
                    panic!("Invalid attribute in {:?}", item.as_rule());
                }
            }
        }
        Some((key, value))
    }
}

#[cfg(test)]
mod tests {
    use crate::ocafile::OCAfileParser;

    use super::*;
    use pest::Parser;

    #[test]
    fn test_add_attribute_instruction() {
        // test vector with example instruction and boolean if they should be valid or not
        let instructions = vec![
            ("ADD ATTRIBUTE documentNumber=Text documentType=Numeric", true),
            ("ADD ATTRIBUTE documentNumber=Text documentType=Numeric name=Text list=Array[Numeric]", true),
            ("ADD ATTRIBUTE name=Text", false),
            ("ADD ATTR name=Text", false),
            ("ADD attribute name=Text", true),
            ("add attribute name=Text", true),
            ("add attribute name=Random", false),
        ];

        // loop over instructions to check if the are meeting the requirements
        for (instruction, is_valid) in instructions {
            let parsed_instruction = OCAfileParser::parse(Rule::add, instruction);

            match parsed_instruction {
                Ok(mut parsed_instruction) => {
                    let instruction = parsed_instruction.next();
                    assert!(instruction.is_some());
                    match instruction {
                        Some(instruction) => {
                            let instruction = AddInstruction::from_record(instruction, 0).unwrap();
                            println!("Parsed instruction: {:?}", instruction);

                            // assert_eq!(instruction.command, Command::Add);
                            // match instruction.data {
                            //     InstructionData::Object(object) => {
                            //         assert_eq!(object.kind, ObjectKind::CaptureBase);
                            //         assert!(object.attributes.len() > 0);
                            //     }
                            //     _ => panic!("Invalid instruction data"),
                            // }
                        }
                        None => {
                            assert!(!is_valid, "Instruction is not valid");
                        }
                    }
                }
                Err(e) => {
                    assert!(!is_valid, "Instruction should be invalid");
                    println!("Error: {:?}", e);
                }
            }
        }
    }
}
