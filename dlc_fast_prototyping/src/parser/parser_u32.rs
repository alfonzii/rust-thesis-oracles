use crate::common::constants::MAX_OUTCOME;
use crate::common::{types, ContractInput, Outcome, OutcomeU32, ParsedContract};
use crate::parser::Parser;

use rayon::prelude::*;
use std::fs;
use std::io::Error;

pub struct SimpleU32Parser;

impl SimpleU32Parser {
    // Creates a Vec of length `len` where v[i] = init + i*inc

    // Serial version in standard Rust:
    fn create_ramp(start_outcome: u32, len: usize, init: u32, inc: i32) -> Vec<(OutcomeU32, u32)> {
        (0..len)
            .map(|i| {
                (
                    OutcomeU32::from(start_outcome + i as u32),
                    (init as i32 + (i as i32 * inc)) as u32,
                )
            })
            .collect()
    }

    // Parallel version using Rayon:
    fn create_ramp_parallel(
        start_outcome: u32,
        len: usize,
        init: u32,
        inc: i32,
    ) -> Vec<(OutcomeU32, u32)> {
        (0..len)
            .into_par_iter() // make a parallel iterator
            .map(|i| {
                (
                    OutcomeU32::from(start_outcome + i as u32),
                    (init as i32 + (i as i32 * inc)) as u32,
                )
            })
            .collect()
    }

    fn create_const_payout_vec(
        start_outcome: u32,
        interval_len: usize,
        constant_payout: u32,
    ) -> Vec<(OutcomeU32, u32)> {
        (0..interval_len)
            .map(|i| (OutcomeU32::from(start_outcome + i as u32), constant_payout))
            .collect()
    }

    fn create_const_payout_vec_parallel(
        start_outcome: u32,
        interval_len: usize,
        constant_payout: u32,
    ) -> Vec<(OutcomeU32, u32)> {
        (0..interval_len)
            .into_par_iter()
            .map(|i| (OutcomeU32::from(start_outcome + i as u32), constant_payout))
            .collect()
    }
}

impl Parser<types::OutcomeU32> for SimpleU32Parser {
    fn parse_input(contract_path: &str) -> Result<ParsedContract<types::OutcomeU32>, Error> {
        // Read file into string
        let contract_input_str =
            fs::read_to_string(contract_path).expect("Error reading contract input file.");

        // Deserialize contract input
        let contract_input: ContractInput =
            serde_json::from_str(&contract_input_str).expect("Error deserializing contract input.");

        let mut result = ParsedContract::<types::OutcomeU32>::with_capacity(MAX_OUTCOME as usize);

        for interval in &contract_input
            .contract_info
            .contract_descriptor
            .payout_intervals
        {
            let start_point = &interval.payout_points[0];
            let end_point = &interval.payout_points[1];

            let start_outcome = start_point.event_outcome;
            let end_outcome = end_point.event_outcome;
            let interval_len = end_outcome.saturating_sub(start_outcome);

            let start_payout = start_point.outcome_payout;
            let end_payout = end_point.outcome_payout;
            let diff = end_payout as i64 - start_payout as i64;

            if diff == 0 {
                let repeated = Self::create_const_payout_vec(
                    start_outcome,
                    interval_len as usize,
                    start_payout,
                );
                result.extend(repeated.into_iter());
            } else {
                let step = diff / interval_len as i64;
                let ramp = Self::create_ramp(
                    start_outcome,
                    interval_len as usize,
                    start_payout,
                    step as i32,
                );
                result.extend(ramp.into_iter());
            }
        }
        Ok(result)
    }
}
