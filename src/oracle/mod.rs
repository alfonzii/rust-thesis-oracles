// src/oracle/mod.rs

use crate::common::OutcomeU32;
use secp256k1_zkp::{PublicKey, SecretKey};

#[allow(dead_code)] // delete if get_public_key used
pub trait Oracle {
    fn get_public_key(&self) -> PublicKey;
    fn get_event_announcement(&self, event_id: u32) -> OracleAnnouncement; // event_id not in use right now, but is here for future experimentations
    fn get_event_attestation(&self, event_id: u32) -> OracleAttestation;
}

pub struct OracleAnnouncement {
    pub public_key: PublicKey,
    pub public_nonce: PublicKey, // INFO: Converted to a single public_nonce instead of a vector of public_nonces (as in `rust-dlc`), as our code currently uses only one nonce and attestation. If we later decide to implement a digit_decomposition approach, both nonce and attestation fields will be transformed into vectors, with the [0] element serving as the default for our approach.
    pub _next_attestation_time: u32, // unix timestamp, INFO: not in use now
}

pub struct OracleAttestation {
    pub outcome: OutcomeU32, // INFO: Oracle will always return outcome in an integer form. We call for it just once, so if we need, we can convert it for negligible perf cost
    pub attestation: SecretKey,
}

mod rand_int_oracle;
pub use rand_int_oracle::RandIntOracle;
