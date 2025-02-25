// src/crypto_utils/mod.rs

use secp256k1_zkp::{PublicKey, SecretKey};

use crate::common::types;

// INFO: This module should be created probably as some independent crate from client and oracle code. It will be included both in client and oracle.
// By doing this, it will be ensured, that both client and oracle use same crypto utility to cumpute anticipation point and to compute attestation on oracle side.

// DISCLAIMER: one thing to bear in mind is, that both computation of anticipation point and attestation must use equivalent algorithms.

pub trait CryptoUtils {
    fn new() -> Self
    where
        Self: Sized;

    fn compute_anticipation_point(
        &self,
        public_key: &PublicKey,
        public_nonce: &PublicKey,
        outcome: &impl types::Outcome,
    ) -> Result<types::AnticipationPoint, secp256k1_zkp::Error>;

    fn compute_attestation(
        &self,
        private_key: &SecretKey,
        private_nonce: &SecretKey,
        outcome: &impl types::Outcome,
    ) -> Result<types::Attestation, secp256k1_zkp::Error>;
}

pub mod secp_utils;
pub mod simple_crypto_utils;

// TODO: mozno by som to mal robit tak, ze parametre necham normalne typy take ake sa pouzivaju, ze nebudem davat types::dacodaco, lebo aj tak su v napovede nazvy lokalnych parametrov (ako public_key, poublic_nonce, outcome).
// ale return by som nechal otypovany aliasom, pretoze return tam neni nazov premmenj/parametru, tak aby bolo z aliasu jasne o aky typ ide. Just saying
