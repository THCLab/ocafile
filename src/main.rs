use std::convert::TryFrom;
use std::fs;

mod error;
mod instructions;
mod ocafile_parser;

use crate::error::Error;
use clap::Parser as ClapParser;
use clap::Subcommand;
use pest::Parser;

use crate::instructions::*;
use crate::ocafile_parser::*;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        file: Option<String>,
    },
    Publish {
        #[arg(short, long)]
        repository: String,
    },
    Sign {
        #[arg(short, long)]
        scid: String,
    },
}

struct OCAfile {
    content: String,
    commands: Vec<Commands>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    From(FromInstruction),
    Add(AddInstruction),
    //    Remove(RemoveInstruction),
    //    Alter(AlterInstruction),
}

impl From<FromInstruction> for Instruction {
    fn from(ins: FromInstruction) -> Self {
        Instruction::From(ins)
    }
}

impl From<AddInstruction> for Instruction {
    fn from(ins: AddInstruction) -> Self {
        Instruction::Add(ins)
    }
}
impl TryFrom<Pair<'_>> for Instruction {
    type Error = crate::Error;

    fn try_from(record: Pair) -> std::result::Result<Self, Self::Error> {
        let instruction: Instruction = match record.as_rule() {
            Rule::from => FromInstruction::from_record(record, 0)?.into(),
            Rule::add => AddInstruction::from_record(record, 0)?.into(),
            _ => return Err(Error::UnexpectedToken(record.to_string())),
        };
        Ok(instruction)
    }
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Build { file }) => {
            println!("Building OCA bundle from oca file");
            let unparsed_file = fs::read_to_string("OCAfile").expect("cannot read file");

            let file = OCAfileParser::parse(Rule::file, &unparsed_file)
                .expect("unsuccessful parse")
                .next()
                .unwrap();

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

                let mut instruction = Instruction::try_from(line).unwrap();
                match &mut instruction {
                    Instruction::From(ref mut from) => {
                        println!("Instruction From");
                    }
                    Instruction::Add(ref mut add) => {
                        println!("Instruction Add");
                    }
                }
            }
        }
        Some(Commands::Publish { repository }) => {
            println!("Publish OCA bundle to repository")
        }
        Some(Commands::Sign { scid }) => {
            println!("Sign OCA bundle byc SCID")
        }
        None => {}
    }

    println!("DONE");
}

// ocafile build -i OCAfile
// ocafile build -s scid
// ocafile publish
// ocafile fetch SAI
// ocafile inspect
