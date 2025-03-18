// all
pub const NB_DIGITS: u8 = 5; // TODO: asi prerobit do usize a takisto navazne funkcie na to (ako napr get_bit)
pub const MAX_OUTCOME: u32 = 15; // 2^NB_DIGITS - 1
pub const TOTAL_COLLATERAL: u64 = 200;

pub const ZERO_OUTCOME_ATP: u32 = 1000; // random value bigger than NB_DIGITS representing zero outcome anticipation point value calc

// main
pub const ALICE: &str = "Alice";
pub const BOB: &str = "Bob";
pub const CONTRACT_INPUT_PATH: &str =
    "./input_contracts/sample_contracts/simple_contract_input.json";

// "./input_contracts/test_contracts/noncontinuous_input.json";
// "./input_contracts/sample_contracts/numerical_contract_input.json";
// 1_048_575
