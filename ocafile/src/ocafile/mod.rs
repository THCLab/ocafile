mod instructions;
mod error;

use self::{instructions::{from::FromInstruction, add::AddInstruction}};
use log::debug;
use ocaast::{OCAAst, Command, CommandType};
use crate::ocafile::error::Error;
use core::convert::From;
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

pub trait TryFromPair {
    type Error;
    fn try_from_pair(pair: Pair<'_>) -> Result<Command, Self::Error>;
}

impl_instruction!(FromInstruction, Instruction::From);
impl_instruction!(AddInstruction, Instruction::Add);

impl TryFromPair for Command {
    type Error = Error;
    fn try_from_pair(record: Pair) -> std::result::Result<Self, Self::Error> {
        let instruction: Command = match record.as_rule() {
            Rule::from => FromInstruction::from_record(record, 0)?.into(),
            Rule::add => AddInstruction::from_record(record, 0)?.into(),
            _ => return Err(Error::UnexpectedToken(record.to_string())),
        };
        Ok(instruction)
    }
}

 /// Parse OCAfile from string and generate OCABox
 pub fn parse_from_string(unparsed_file: String) -> OCAAst {
    let file = OCAfileParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut oca_ast = OCAAst::new();


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

        let command = match Command::try_from_pair(line) {
            Ok(command) => {
                oca_ast.commands.push(command);
            },
            Err(e) => {
                panic!("Error parsing instruction: {}", e);
            }
        };

        // match &mut command.kind {
        //     CommandType::From => {
        //         debug!(
        //             "NOT IMPLEMENTED YET: Searching OCA bundle from available sources: {:?}",
        //             command.content
        //         );
        //         // load new OCABundle from repository and create instance object of it
        //     }
        //     CommandType::Add => {
        //         // Convert instruction AST into OCABox

        //         // for attribute in instruction.attributes.iter() {
        //         //     debug!("Adding attribute to bundle: {:?}", attribute);
        //         //     oca_box.add_attribute(attribute.clone());
        //         // }
        //     }
        //     CommandType::Remove => todo!(),
        //     CommandType::Modify => todo!(),
        // }


        // Each instruction should generate hash of the OCA bundle at given point and all it's oca objects
        // this would be used in OCA repository for matching OCA bundles and searching for them
        // generate said of the ocabundle and store in local db
        // let said = oca_box.get_bundle_said();

    }
    return oca_ast;
}