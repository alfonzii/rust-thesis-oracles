// src/oracle/mod.rs

use crate::common::{types, OutcomeU32};

pub trait Oracle {
    fn get_public_key(&self) -> types::PublicKey;
    fn get_event_announcement(
        &self,
        event_id: u32,
    ) -> OracleAnnouncement<types::PublicKey, types::PublicNonce>;
    fn get_event_attestation(&self, event_id: u32) -> OracleAttestation;
}

pub struct OracleAnnouncement<PK, PN> {
    pub public_key: PK,
    pub public_nonces: Vec<PN>,
    pub next_attestation_time: u32, // unix timestamp
}

pub struct OracleAttestation {
    pub outcome: OutcomeU32, // INFO: Oracle will always return outcome in an integer form. We call for it just once, so if we need, we can convert it for negligible perf cost
    pub attestation: types::Attestation, // TODO: asi z toho urobit tiez pole, lebo ich rust-dlc algoritmus pouziva prave "pocetbitov" atestacii
}

mod rand_int_oracle;
pub use rand_int_oracle::RandIntOracle;
