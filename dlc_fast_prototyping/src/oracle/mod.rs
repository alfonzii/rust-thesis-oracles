// src/oracle/mod.rs

pub trait Oracle {
    type PublicKey; // Point
    type PubNonce; // Point
    type Outcome;
    type Attestation; // Scalar

    fn get_public_key(&self) -> Vec<Self::PublicKey>; // vector; just so we can test multipub oracle later. in normal scenario we use just one pubkey (first element)
    fn get_announcement(&self, event_id: u32) -> (Vec<Self::PublicKey>, Vec<Self::PubNonce>, u32); // u32 in result stands for closest following attestation time
    fn get_attestation(&self, event_id: u32) -> (Self::Outcome, Self::Attestation);
}

mod rand_int_oracle;
pub use rand_int_oracle::RandIntOracle;
