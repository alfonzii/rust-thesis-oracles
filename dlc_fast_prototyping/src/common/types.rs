// src/common/types.rs

use secp256k1_zkp;
use secp256k1_zkp::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};

use crate::common::error::ContractError;
use crate::config::NB_DIGITS;

/// -- Aliases for outcome types --
pub type AnticipationPoint = PublicKey;
pub type Attestation = SecretKey;

// Other
pub type Cet = String; // Contract Execution Transaction (esentially not signed Tx)
pub type PayoutT = u64; // Integer type for payout values (in satoshis). If will be using Bitcoin API, then Amount will be correct type
pub type ParsedContract<O: Outcome> = Vec<(O, PayoutT)>; // (outcome, payout) pairs.. TODO: nejak to treba vymysliet aby to slo urobit
                                                         // TODO: za predpokladu, ze ParsedContract bude obsahovat len OutcomeU32, tak by to mohol byt iba Vec<u32>

/// The final Bitcoin transaction or any other on-chain transaction type
/// that will be broadcasted after finalization.
pub struct FinalizedTx<Sig> {
    pub payload: Cet,
    pub signature1: Sig,
    pub signature2: Sig,
}

impl<Sig> FinalizedTx<Sig> {
    pub fn new(payload: Cet, signature1: Sig, signature2: Sig) -> Self {
        Self {
            payload,
            signature1,
            signature2,
        }
    }
}

pub struct MultisigFundAddress {
    pub public_key1: PublicKey,
    pub public_key2: PublicKey,
}

impl MultisigFundAddress {
    pub fn new(public_key1: PublicKey, public_key2: PublicKey) -> Self {
        Self {
            public_key1,
            public_key2,
        }
    }
}

// ------------------ Outcome trait and implementations ------------------
pub trait Outcome {
    type ValueType;

    /// Return the value of this Outcome.
    fn get_value(&self) -> Self::ValueType;

    /// Return bit at the given position.
    fn get_bit(&self, position: u8) -> bool;

    /// Return true if the outcome is zero.
    fn is_zero(&self) -> bool;

    /// Serialize this Outcome into bytes to store or transmit.
    fn serialize(&self) -> Vec<u8>; // TODO: will need to remake to [u8; N], because we will be creating lots of small vectors. Instead, we can cap size of String to be equal to size of u32 and then, we know how big to return.
}

// A simple integer-based outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutcomeU32 {
    value: u32,
}

impl Outcome for OutcomeU32 {
    type ValueType = u32;

    fn get_value(&self) -> u32 {
        self.value
    }

    fn get_bit(&self, position: u8) -> bool {
        debug_assert!(position < 32, "Position must be less than 32");
        (self.value >> position) & 1 == 1
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn serialize(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }
}

impl From<u32> for OutcomeU32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<OutcomeU32> for u32 {
    fn from(outcome: OutcomeU32) -> u32 {
        outcome.value
    }
}

// A string-based binary outcome
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutcomeBinStr {
    value: String,
}

impl Outcome for OutcomeBinStr {
    type ValueType = String;

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn get_bit(&self, position: u8) -> bool {
        debug_assert!(
            position < self.value.len() as u8,
            "Position must be less than the length of the string"
        );
        self.value.chars().nth(position as usize) == Some('1')
    }

    fn is_zero(&self) -> bool {
        self.value.chars().all(|c| c == '0')
    }

    fn serialize(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }
}

impl From<String> for OutcomeBinStr {
    fn from(value: String) -> Self {
        // TODO: theoretically, bcs of performance, if we can assume correct string, doesn't have to be here
        if !value.chars().all(|c| c == '0' || c == '1') {
            panic!("OutcomeBinStr can only contain '0' and '1' characters.");
        }
        Self { value }
    }
}

impl From<OutcomeBinStr> for String {
    fn from(outcome: OutcomeBinStr) -> Self {
        outcome.value
    }
}

// ------------------ ContractInput and related structs ------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractInput {
    pub offer_collateral: u64,  // Amount (btc cargo)
    pub accept_collateral: u64, // Amount (btc cargo)
    pub fee_rate: u64,
    pub contract_info: ContractInfo,
}

impl ContractInput {
    pub fn validate(&self) -> Result<(), ContractError> {
        // 10: Input contract must be non-empty
        if self.offer_collateral == 0 || self.accept_collateral == 0 || self.fee_rate == 0 {
            return Err(ContractError::EmptyContract);
        }

        // 6: Collateral must be valid unsigned integer (secured by u64)
        /*if self.offer_collateral < 0 || self.accept_collateral < 0 {
            return Err(ContractError::InvalidCollateral);
        }*/

        // 7: If feeRate > 25 * 250 => error
        if self.fee_rate > 25 * 250 {
            return Err(ContractError::TooHighFeeRate);
        }

        // Validate the rest
        let sum_collaterals = self.offer_collateral + self.accept_collateral;
        self.contract_info.validate(sum_collaterals)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfo {
    pub contract_descriptor: ContractDescriptor,
    pub oracle: OracleInput,
}

impl ContractInfo {
    pub fn validate(&self, max_payout: u64) -> Result<(), ContractError> {
        self.oracle.validate()?;
        self.contract_descriptor
            .validate(max_payout, self.oracle.nb_digits)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDescriptor {
    pub payout_intervals: Vec<PayoutInterval>,
}

impl ContractDescriptor {
    pub fn validate(&self, max_payout: u64, nb_digits: u8) -> Result<(), ContractError> {
        // 9: must have at least one interval
        if self.payout_intervals.is_empty() {
            return Err(ContractError::MissingIntervals);
        }

        // 2^NB_DIGITS - 1 is expected final outcome
        let expected_final_outcome = (1 << nb_digits) - 1;

        // 2: First point of first interval must start at zero
        let first_interval = &self.payout_intervals[0];
        let first_pt = first_interval
            .payout_points
            .get(0)
            .ok_or(ContractError::InvalidIntervalPoints)?;
        if first_pt.event_outcome != 0 {
            return Err(ContractError::InvalidFirstOutcome);
        }

        // 1. Validate each interval, check continuity
        for w in self.payout_intervals.windows(2) {
            let end_of_this = w[0].payout_points[1].event_outcome;
            let start_of_next = w[1].payout_points[0].event_outcome;
            if end_of_this != start_of_next {
                return Err(ContractError::NonContinuousIntervals);
            }
        }

        // 3: Last point must end on 2^NB_DIGITS - 1
        let last_interval = self.payout_intervals.last().unwrap();
        let last_pt = last_interval
            .payout_points
            .get(1)
            .ok_or(ContractError::InvalidIntervalPoints)?;
        if last_pt.event_outcome != expected_final_outcome {
            return Err(ContractError::OutcomeRangeMismatch);
        }

        // Validate intervals individually
        for interval in &self.payout_intervals {
            interval.validate(max_payout)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutInterval {
    pub payout_points: Vec<PayoutPoint>,
}

impl PayoutInterval {
    pub fn validate(&self, max_payout: u64) -> Result<(), ContractError> {
        // 8: Each interval should have exactly 2 points
        if self.payout_points.len() != 2 {
            return Err(ContractError::InvalidIntervalPoints);
        }
        // Validate each payout point
        for point in &self.payout_points {
            point.validate(max_payout)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutPoint {
    pub event_outcome: u32,
    pub outcome_payout: PayoutT, // Amount (btc cargo)
}

impl PayoutPoint {
    pub fn validate(&self, max_payout: u64) -> Result<(), ContractError> {
        // 5: outcomePayout must be non-negative (already guaranteed by u64)
        /*if self.outcome_payout < 0 {
            return Err(ContractError::NegativePayout);
        }*/

        // 4: outcomePayout <= sum of offerCollateral and acceptCollateral
        if self.outcome_payout > max_payout {
            return Err(ContractError::InvalidPayout);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleInput {
    pub public_key: PublicKey,
    pub event_id: String,
    pub nb_digits: u8, // TODO: asi potom tiez prerobit na usize
}

impl OracleInput {
    pub fn validate(&self) -> Result<(), ContractError> {
        // 11. nb_digits from contract must match NB_DIGITS constant
        if self.nb_digits != NB_DIGITS {
            return Err(ContractError::NbDigitsMismatch);
        }
        Ok(())
    }
}

// TODO: written here but applies to whole project! might think about if renaming isnt needed. we are using whole names like public_key, private_key... maybe using just priv_key and pub_key would be enough and more readable.
