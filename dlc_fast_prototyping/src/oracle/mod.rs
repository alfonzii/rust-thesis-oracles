// src/oracle/mod.rs

use crate::common::OutcomeU32;
use secp256k1_zkp::{PublicKey, SecretKey};

pub trait Oracle {
    fn get_public_key(&self) -> PublicKey;
    fn get_event_announcement(&self, event_id: u32) -> OracleAnnouncement;
    fn get_event_attestation(&self, event_id: u32) -> OracleAttestation;
}

pub struct OracleAnnouncement {
    pub public_key: PublicKey,
    pub public_nonces: Vec<PublicKey>,
    pub next_attestation_time: u32, // unix timestamp
}

pub struct OracleAttestation {
    pub outcome: OutcomeU32, // INFO: Oracle will always return outcome in an integer form. We call for it just once, so if we need, we can convert it for negligible perf cost
    pub attestation: SecretKey, // TODO: asi z toho urobit pole, lebo ich rust-dlc algoritmus pouziva prave "pocetbitov" atestacii
}

mod rand_int_oracle;
pub use rand_int_oracle::RandIntOracle;
