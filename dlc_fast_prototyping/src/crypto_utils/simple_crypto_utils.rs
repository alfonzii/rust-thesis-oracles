use crate::{common::types, crypto_utils::secp_utils::*};
use secp256k1_zkp::{PublicKey, SecretKey, SECP256K1};

use super::CryptoUtils;

pub struct SimpleCryptoUtils;

impl CryptoUtils for SimpleCryptoUtils {
    fn new() -> Self {
        Self
    }

    fn compute_anticipation_point(
        &self,
        public_key: &PublicKey,
        public_nonce: &PublicKey,
        outcome: &impl types::Outcome,
    ) -> Result<types::AnticipationPoint, secp256k1_zkp::Error> {
        schnorrsig_compute_anticipation_point(SECP256K1, public_key, public_nonce, outcome)
    }

    fn compute_attestation(
        &self,
        private_key: &SecretKey,
        private_nonce: &SecretKey,
        outcome: &impl types::Outcome,
    ) -> Result<types::Attestation, secp256k1_zkp::Error> {
        schnorrsig_compute_oracle_attestation(SECP256K1, private_key, private_nonce, outcome)
    }
}

// TODO: this file needs to be merged with secp_utils.rs, The current structure is not optimal.
