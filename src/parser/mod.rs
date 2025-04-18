//! Parser implementations for contract inputs.

use crate::common::{types, ContractInput, ParsedContract};

pub trait Parser<Out: types::Outcome> {
    fn read_input(contract_path: &str) -> Result<ContractInput, std::io::Error>;
    fn parse_contract_input(
        contract_input: ContractInput,
    ) -> Result<ParsedContract<Out>, std::io::Error>;
}

pub mod parser_mock;
pub mod parser_out_u32;
