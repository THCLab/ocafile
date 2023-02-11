use oca_rs::state::attribute::{Attribute, AttributeType};

use crate::error::Error;
use crate::ocafile_parser::*;
use std::str::FromStr;

// enum for ocaobject
enum OCAObject {
    CaptureBase,
    Overlay,
}

pub struct AddInstruction {
    pub attributes: Vec<Attribute>, // what is it is not attribute? check what options we have and which instruction we need to handle
}

impl AddInstruction {
    pub(crate) fn from_record(record: Pair, index: usize) -> Result<AddInstruction, Error> {
        let mut add_instruction = AddInstruction {
            attributes: Vec::new(),
        };

        debug!("{}", record);
        for object in record.into_inner() {
            match object.as_rule() {
                Rule::meta => {
                    for attrs in object.into_inner() {
                        match attrs.as_rule() {
                            Rule::key_pairs => {
                                println!("Meta attr ----> {:?}", attrs);
                            }
                            Rule::lang => {
                                debug!("Parsing language: {:?}", attrs.as_rule())
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
                    for attr_pairs in object.into_inner() {
                        match attr_pairs.as_rule() {
                            Rule::attr_pairs => {
                                info!("attribute: {:?}", attr_pairs);
                                for attr in attr_pairs.into_inner() {
                                    debug!("Parsing attribute {:?}", attr);
                                    add_instruction.extract_attribute(attr);
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

        Ok(add_instruction)
    }

    fn extract_attribute(&mut self, attr_pair: Pair) -> () {
        let mut attribute = None;
        for item in attr_pair.into_inner() {
            match item.as_rule() {
                Rule::key => {
                    attribute = Some(Attribute::new(item.as_str().to_string()));
                }
                Rule::attr_type => match AttributeType::from_str(&item.as_span().as_str()) {
                    Ok(attr_type) => {
                        debug!("Attribute type: {:?}", attr_type);
                        if let Some(attribute) = attribute.as_mut() {
                            attribute.set_attribute_type(attr_type);
                        }
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
        self.attributes.push(attribute.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ocafile_parser::OCAfileParser;
    use pest::Parser;

    #[test]
    fn test_add_instruction() {
        let ocafile = "ADD ATTRIBUTE documentNumber=Text documentType=Numeric";

        let instruction = OCAfileParser::parse(Rule::add, ocafile)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let add_instruction = AddInstruction::from_record(instruction, 0).unwrap();
        assert_eq!(add_instruction.attributes.len(), 2);
        assert_eq!(add_instruction.attributes[0].name, "documentNumber");
        match add_instruction.attributes[0].attribute_type {
            Some(AttributeType::Text) => {
                assert!(true);
            }
            Some(AttributeType::Numeric) => {
                assert!(true);
            }
            _ => panic!("Invalid attribute type"),
        }

    }
}
