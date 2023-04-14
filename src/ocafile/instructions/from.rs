use crate::ocafile::{
    ast::{Command, Instruction, InstructionData},
    error::Error,
    Pair, Rule,
};
use log::debug;
use said::prefix::SelfAddressingPrefix;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FromInstruction {}

impl FromInstruction {
    pub(crate) fn from_record(record: Pair, index: usize) -> Result<Instruction, Error> {
        let mut said_str = None;

        for field in record.into_inner() {
            match field.as_rule() {
                Rule::from_said => said_str = Some(field),
                Rule::comment => continue,
                _ => {
                    return Err(Error::UnexpectedToken(format!(
                        "unexpected token {:?}",
                        field.as_rule()
                    )))
                }
            };
        }

        let said = SelfAddressingPrefix::from_str(said_str.unwrap().as_str()).unwrap();
        debug!("Using oca bundle from: {:?}", said);
        Ok(Instruction {
            command: Command::From,
            data: InstructionData::From(said.to_str()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ocafile::{error::Error, OCAfileParser, Pair, Rule};
    use pest::Parser;
    use pretty_assertions::assert_eq;

    pub fn parse_direct<T, F>(input: &str, rule: Rule, func: F) -> Result<T, Error>
    where
        F: Fn(Pair) -> Result<T, Error>,
    {
        let pair = OCAfileParser::parse(rule, input)
            .expect("unsuccessful parse")
            .next()
            .ok_or(Error::UnexpectedToken("Unknown parser error".to_string()))?;

        func(pair)
    }

    use super::*;

    #[test]
    fn test_from_instruction() -> Result<(), Error> {

        // test vector with example instruction and boolean if they should be valid or not
        let instructions = vec![
                ("FROM E2oRZ5zEKxTfTdECW-v2Q7bM_H0OD0ko7IcCwdo_u9co", true),
                ("from E2oRZ5zEKxTfTdECW-v2Q7bM_H0OD0ko7IcCwdo_u9co", true),
                ("from error", false),
                ("from https://humancolossus.org/E2oRZ5zEKxTfTdECW-v2Q7bM_H0OD0ko7IcCwdo_u9co", false),
            ];

        for (instruction, is_valid) in instructions {
            let result = parse_direct(instruction, Rule::from, |p| {
                FromInstruction::from_record(p, 0)
            });

            match result {
                Ok(_) => {
                    let said =
                    SelfAddressingPrefix::from_str(instruction).unwrap();

                    assert_eq!(from, FromInstruction { said });
                }
                Err(e) => {
                    assert!(!is_valid, "Instruction should be invalid")
                }
            }

        }
        let from = parse_direct(
            "from E2oRZ5zEKxTfTdECW-v2Q7bM_H0OD0ko7IcCwdo_u9co",
            Rule::from,
            |p| FromInstruction::from_record(p, 0),
        )?;

        Ok(())
    }
}
