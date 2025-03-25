use crate::common::constants::MAX_OUTCOME;
use crate::common::{types, OutcomeU32, ParsedContract};
use crate::parser::Parser;

pub struct MockU32Parser;

impl Parser<types::OutcomeU32> for MockU32Parser {
    fn read_input(_contract_path: &str) -> Result<types::ContractInput, std::io::Error> {
        todo!()
    }

    fn parse_contract_input(
        _contract_input: types::ContractInput,
    ) -> Result<ParsedContract<types::OutcomeU32>, std::io::Error> {
        Ok((0..=MAX_OUTCOME)
            .map(|i| (OutcomeU32::from(i), i.into()))
            .collect())
    }
}
