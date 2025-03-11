use crate::common::constants::MAX_OUTCOME;
use crate::common::{types, OutcomeU32, ParsedContract};
use crate::parser::Parser;
use std::io::Error;

pub struct MockU32Parser;

impl Parser<types::OutcomeU32> for MockU32Parser {
    fn parse_input(_contract_path: &str) -> Result<ParsedContract<types::OutcomeU32>, Error> {
        Ok((0..=MAX_OUTCOME - 1)
            .map(|i| (OutcomeU32::from(i), i.into()))
            .collect())
    }
}
