use crate::common::{types, ParsedContract};

pub trait Parser<Out: types::Outcome> {
    // Parses input from a given contract path.
    fn parse_input(contract_path: &str) -> Result<ParsedContract<Out>, std::io::Error>;
}

pub mod parser_mock;
pub mod parser_out_u32;
