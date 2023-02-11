use std::str::FromStr;

use said::prefix::SelfAddressingPrefix;

use crate::error::Error;
use crate::ocafile_parser::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FromInstruction {
    pub said: SelfAddressingPrefix,
}

impl FromInstruction {
    pub(crate) fn from_record(record: Pair, index: usize) -> Result<FromInstruction, Error> {
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
        // TODO retrive OCA bundle from local repository or remote
        Ok(FromInstruction { said })
    }
}

#[cfg(test)]
mod tests {

    use crate::error::Error;
    use crate::ocafile_parser::*;
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
    fn from_said() -> Result<(), Error> {
        let from = parse_direct(
            "from E2oRZ5zEKxTfTdECW-v2Q7bM_H0OD0ko7IcCwdo_u9co",
            Rule::from,
            |p| FromInstruction::from_record(p, 0),
        )?;


        let said = SelfAddressingPrefix::from_str("E2oRZ5zEKxTfTdECW-v2Q7bM_H0OD0ko7IcCwdo_u9co").unwrap();

        assert_eq!(
            from,
            FromInstruction {
                said
            }
        );

        Ok(())
    }
}
