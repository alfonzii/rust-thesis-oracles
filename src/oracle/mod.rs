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
    pub public_nonce: PublicKey, // INFO: Prerobil som to na public_nonce namiesto vektoru public_nonces, pretoze v nasom kode nepoouzivame viac noncov ani atestacii. Ak by sme sa nakoniec predsa rozhodli implementovat aj to ich riesenie s digit_decomposition, tak sa nonce aj atestacie prerobia na vector, a v asom pripade budu vraciat [0] prvok.
    pub _next_attestation_time: u32, // unix timestamp, INFO: not in use now
}

pub struct OracleAttestation {
    pub outcome: OutcomeU32, // INFO: Oracle will always return outcome in an integer form. We call for it just once, so if we need, we can convert it for negligible perf cost
    pub attestation: SecretKey,
}

mod rand_int_oracle;
pub use rand_int_oracle::RandIntOracle;
