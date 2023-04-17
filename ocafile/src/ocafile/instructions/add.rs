use crate::ocafile::{error::Error, Pair, Rule};
use indexmap::IndexMap;
use log::{debug, info};
use oca_rs::state::{attribute::AttributeType};
use ocaast::{Command, CommandType, Content, NestedValue, ObjectKind};
use std::str::FromStr;

pub struct AddInstruction {}

impl AddInstruction {
    pub(crate) fn from_record(record: Pair, _index: usize) -> Result<Command, Error> {
        // let mut nested_object = None;
        let mut object_kind = None;
        let kind = CommandType::Add;
        let mut content = None;

        debug!("Into the record: {:?}", record);
        for object in record.into_inner() {
            match object.as_rule() {
                Rule::meta => {
                    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
                    object_kind = Some(ObjectKind::Overlay(ocaast::OverlayType::Meta));
                    for attr_pairs in object.into_inner() {
                        match attr_pairs.as_rule() {
                            Rule::key_pairs => {
                                // println!("Meta attr ----> {:?}", attrs);
                                for attr in attr_pairs.into_inner() {
                                    debug!("Parsing meta attribute {:?}", attr);
                                    if let Some((key, value)) =
                                        AddInstruction::extract_attribute(attr)
                                    {
                                        debug!("Parsed meta attribute: {:?} = {:?}", key, value);
                                        properties.insert(key, NestedValue::Value(value));
                                    } else {
                                        debug!("Skipping meta attribute");
                                    }
                                }
                            }
                            Rule::lang => {
                                debug!("Parsing language: {:?}", attr_pairs.as_rule());
                                properties.insert(
                                    "lang".to_string(),
                                    NestedValue::Value(attr_pairs.as_str().to_string()),
                                );
                            }
                            _ => {
                                return Err(Error::UnexpectedToken(format!(
                                    "Invalid attribute in meta overlay {:?}",
                                    attr_pairs.as_rule()
                                )))
                            }
                        }
                    }
                    content = Some(Content {
                        properties: Some(properties),
                        attributes: None,
                    });
                }
                Rule::attribute => {
                    object_kind = Some(ObjectKind::CaptureBase);
                    let mut attributes: IndexMap<String, NestedValue> = IndexMap::new();
                    for attr_pairs in object.into_inner() {
                        match attr_pairs.as_rule() {
                            Rule::attr_pairs => {
                                info!("attribute: {:?}", attr_pairs);
                                for attr in attr_pairs.into_inner() {
                                    debug!("Parsing attribute {:?}", attr);
                                    if let Some((key, value)) =
                                        AddInstruction::extract_attribute(attr)
                                    {
                                        debug!("Parsed attribute: {:?} = {:?}", key, value);

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
                    content = Some(Content {
                        properties: None,
                        attributes: Some(attributes),
                    });
                }
                Rule::comment => continue,
                Rule::classification => {
                    object_kind = Some(ObjectKind::CaptureBase);
                    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
                    let classification = object.into_inner().next().unwrap();
                    print!("Classification: {:?}", classification.as_rule());
                    properties.insert(
                        "classification".to_string(),
                        NestedValue::Value(classification.as_str().to_string()),
                    );

                    content = Some(Content {
                        properties: Some(properties),
                        attributes: None,
                    });

                }
                Rule::information => {
                    object_kind = Some(ObjectKind::Overlay(ocaast::OverlayType::Information))
                },
                Rule::character_encoding => {
                    object_kind = Some(ObjectKind::Overlay(ocaast::OverlayType::CharacterEncoding))
                },
                Rule::character_encoding_props => {
                    object_kind = Some(ObjectKind::Overlay(ocaast::OverlayType::CharacterEncoding))
                },
                Rule::label => {
                    object_kind = Some(ObjectKind::Overlay(ocaast::OverlayType::Label))
                },
                Rule::unit => {
                    object_kind = Some(ObjectKind::Overlay(ocaast::OverlayType::Unit))
                },
                Rule::format => {
                    object_kind = Some(ObjectKind::Overlay(ocaast::OverlayType::Format))
                },
                Rule::flagged_attrs => {
                    object_kind = Some(ObjectKind::CaptureBase)
                },
                _ => {
                    return Err(Error::UnexpectedToken(format!(
                        "Overlay: unexpected token {:?}",
                        object.as_rule()
                    )))
                }
            };
        }

        Ok(Command {
            kind: kind,
            object_kind: object_kind.unwrap(),
            content: content,
        })
    }

    fn extract_attribute(attr_pair: Pair) -> Option<(String, String)> {
        let mut key = String::new();
        let mut value = String::new();

        debug!("Extract the attribute: {:?}", attr_pair);
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
                Rule::key_value => {
                    value = item.as_str().to_string();
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

                            assert_eq!(instruction.kind, CommandType::Add);
                            assert_eq!(instruction.object_kind, ObjectKind::CaptureBase);
                            match instruction.content {
                                Some(content) => {
                                    assert!(content.attributes.is_some());
                                    assert!(content.attributes.unwrap().len() > 0);
                                }
                                None => {
                                    assert!(!is_valid, "Instruction is not valid");
                                }
                            }
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
