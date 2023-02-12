mod instructions;
mod error;

use self::instructions::{from::FromInstruction, add::AddInstruction};
use crate::ocafile::error::Error;
use core::convert::From;
use log::debug;
use oca_rs::state::oca::OCABox;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "ocafile.pest"]
pub struct OCAfileParser;

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

enum Instruction {
    From(FromInstruction),
    Add(AddInstruction),
    //    Remove(RemoveInstruction),
    //    Alter(AlterInstruction),
}

macro_rules! impl_instruction {
    ($struct:ident, $enum:expr) => {
        impl From<$struct> for Instruction {
            fn from(instruction: $struct) -> Self {
                $enum(instruction)
            }
        }
    };
}

impl_instruction!(FromInstruction, Instruction::From);
impl_instruction!(AddInstruction, Instruction::Add);

impl TryFrom<Pair<'_>> for Instruction {
    type Error = Error;
    fn try_from(record: Pair) -> std::result::Result<Self, Self::Error> {
        let instruction: Instruction = match record.as_rule() {
            Rule::from => FromInstruction::from_record(record, 0)?.into(),
            Rule::add => AddInstruction::from_record(record, 0)?.into(),
            _ => return Err(Error::UnexpectedToken(record.to_string())),
        };
        Ok(instruction)
    }
}

 /// Parse OCAfile from string and generate OCABox
 pub fn parse_from_string(unparsed_file: String) -> OCABox {
    let file = OCAfileParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut oca_box = OCABox::new();

    for line in file.into_inner() {
        if let Rule::EOI = line.as_rule() {
            continue;
        }
        if let Rule::comment = line.as_rule() {
            continue;
        }
        if let Rule::empty_line = line.as_rule() {
            continue;
        }

        let mut instruction = match Instruction::try_from(line) {
            Ok(instruction) => instruction,
            Err(e) => {
                panic!("Error parsing instruction: {}", e);
            }
        };

        match &mut instruction {
            Instruction::From(ref mut from) => {
                debug!(
                    "NOT IMPLEMENTED YET: Searching OCA bundle from available sources: {:?}",
                    from.said
                );
                // load new OCABundle from repository and create instance object of it
            }
            Instruction::Add(ref mut instruction) => {
                for attribute in instruction.attributes.iter() {
                    debug!("Adding attribute to bundle: {:?}", attribute);
                    oca_box.add_attribute(attribute.clone());
                }
            }
        }
    }
    return oca_box;
}