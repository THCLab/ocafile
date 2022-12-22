use crate::error::Error;
use crate::ocafile_parser::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AddInstruction {
    attributes: HashMap<String, String>,
}

impl AddInstruction {
    pub(crate) fn from_record(record: Pair, index: usize) -> Result<AddInstruction, Error> {
        let mut attributes = HashMap::new();
        let mut overlay = ""; // Should be overlay object
        let overlay = "";

        //       println!("{:?}", record);
        // TODO
        for overlay in record.into_inner() {
            match overlay.as_rule() {
                Rule::meta => {
                    //                    println!(" META: {:?}", overlay);
                    for attrs in overlay.into_inner() {
                        println!("--- Overlay ---");
                        println!("{:?}", attrs);
                        println!("---- END ---");
                        match attrs.as_rule() {
                            Rule::attr_pairs => {
                                println!("Meta attr ----> {:?}", attrs);
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
                Rule::comment => continue,
                _ => {
                    return Err(Error::UnexpectedToken(format!(
                        "Overlay: unexpected token {:?}",
                        overlay.as_rule()
                    )))
                }
            };
        }

        Ok(AddInstruction {
            attributes: attributes,
        })
    }
}
