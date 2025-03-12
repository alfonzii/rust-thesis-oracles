use crate::common::constants::MAX_OUTCOME;
use crate::common::{types, ContractInput, Outcome, OutcomeU32, ParsedContract};
use crate::parser::Parser;

use rayon::prelude::*;
use std::fs;
use std::io::Error;

pub struct SimpleOutU32Parser;

impl SimpleOutU32Parser {
    // Creates a Vec of length `len` where v[i] = init + i*inc

    // Serial version in standard Rust:
    fn create_linear_seq(
        start_outcome: u32,
        len: u32,
        start_payout: types::PayoutT,
        step: f64,
    ) -> Vec<(types::OutcomeU32, types::PayoutT)> {
        let start_payout_f = start_payout as f64;
        (0..len)
            .map(|i| {
                (
                    OutcomeU32::from(start_outcome + i),
                    (start_payout_f + (i as f64 * step)).round() as u64,
                )
            })
            .collect()
    }

    // Parallel version using Rayon:
    fn create_linear_seq_parallel(
        start_outcome: u32,
        len: u32,
        start_payout: types::PayoutT,
        step: f64,
    ) -> Vec<(types::OutcomeU32, types::PayoutT)> {
        let start_payout_f = start_payout as f64;
        (0..len)
            .into_par_iter() // make a parallel iterator
            .map(|i| {
                (
                    OutcomeU32::from(start_outcome + i),
                    (start_payout_f + (i as f64 * step)).round() as u64,
                )
            })
            .collect()
    }

    fn create_const_payout_vec(
        start_outcome: u32,
        interval_len: u32,
        constant_payout: types::PayoutT,
    ) -> Vec<(types::OutcomeU32, types::PayoutT)> {
        (0..interval_len)
            .map(|i| (OutcomeU32::from(start_outcome + i), constant_payout))
            .collect()
    }

    fn create_const_payout_vec_parallel(
        start_outcome: u32,
        interval_len: u32,
        constant_payout: types::PayoutT,
    ) -> Vec<(types::OutcomeU32, types::PayoutT)> {
        (0..interval_len)
            .into_par_iter()
            .map(|i| (OutcomeU32::from(start_outcome + i), constant_payout))
            .collect()
    }
}

impl Parser<types::OutcomeU32> for SimpleOutU32Parser {
    fn parse_input(contract_path: &str) -> Result<ParsedContract<types::OutcomeU32>, Error> {
        // Read input file containing contract JSON into string
        let contract_input_str =
            fs::read_to_string(contract_path).expect("Error reading contract input file.");

        // Deserialize contract input
        let contract_input: ContractInput =
            serde_json::from_str(&contract_input_str).expect("Error deserializing contract input.");

        // Reserve capacity for final vector based on maximum possible outcome (we avoid reallocating by doing this)
        let mut parsed_contract =
            ParsedContract::<types::OutcomeU32>::with_capacity(MAX_OUTCOME as usize);

        // Parse contract intervals and create a vector of (outcome, payout) pairs
        for interval in &contract_input
            .contract_info
            .contract_descriptor
            .payout_intervals
        {
            // Get start and end points of the interval
            let start_point = &interval.payout_points[0];
            let end_point = &interval.payout_points[1];

            // Calculate interval length and payout difference
            let start_outcome = start_point.event_outcome;
            let end_outcome = end_point.event_outcome;
            let interval_len = end_outcome - start_outcome;

            let start_payout = start_point.outcome_payout;
            let end_payout = end_point.outcome_payout;
            let diff = (end_payout as i64) - (start_payout as i64);

            if diff == 0 {
                let repeated =
                    Self::create_const_payout_vec(start_outcome, interval_len, start_payout); // Might also use end_payout. It doesn't matter as both are same
                parsed_contract.extend(repeated.into_iter());
            } else {
                let step: f64 = (diff as f64) / (interval_len as f64);
                let linear_seq =
                    Self::create_linear_seq(start_outcome, interval_len, start_payout, step);
                parsed_contract.extend(linear_seq.into_iter());
            }
        }

        // Manually add the last interval's last point to the parsed contract, as we don't add into `repeated` last (outcome, payout) pair from interval
        let last_interval = contract_input
            .contract_info
            .contract_descriptor
            .payout_intervals
            .last()
            .expect("No payout intervals found in contract input");
        let last_point = last_interval
            .payout_points
            .last()
            .expect("No payout points found in the last interval");
        parsed_contract.push((
            OutcomeU32::from(last_point.event_outcome),
            last_point.outcome_payout,
        ));

        Ok(parsed_contract)
    }
}
