use secp256k1_zkp::{
    global::SECP256K1,
    rand::{thread_rng, Rng},
    Keypair, PublicKey, SecretKey,
};

use crate::oracle;
use crate::secp_utils; // Ensure the correct path to the oracle module

pub struct RandIntOracle {
    nonces: Keypair,
    keys: Keypair,
    outcome: u32, // Simple u32 value
}

impl RandIntOracle {
    pub fn new() -> Self {
        let nonces = Keypair::new(SECP256K1, &mut thread_rng());
        let keys = Keypair::new(SECP256K1, &mut thread_rng());

        let mut rng = thread_rng();
        let outcome: u32 = rng.gen();

        Self {
            nonces,
            keys,
            outcome,
        }
    }

    pub fn get_outcome(&self) -> u32 {
        self.outcome
    }

    fn compute_attestation(&self) -> SecretKey {
        let priv_nonce = self.nonces.secret_key();
        let priv_key = self.keys.secret_key();

        secp_utils::schnorrsig_compute_oracle_attestation(
            &SECP256K1,
            &priv_key,
            &priv_nonce,
            self.outcome,
        )
        .unwrap()
    }
}

// Implement the Oracle trait for RandIntOracle
impl oracle::Oracle for RandIntOracle {
    type PublicKey = PublicKey;
    type PubNonce = PublicKey;
    type Outcome = u32;
    type Attestation = SecretKey;

    fn get_public_key(&self) -> Vec<Self::PublicKey> {
        vec![self.keys.public_key()]
    }

    fn get_announcement(&self, _event_id: u32) -> (Vec<Self::PublicKey>, Vec<Self::PubNonce>, u32) {
        let pub_keys = vec![self.keys.public_key()];
        let pub_nonces = vec![self.nonces.public_key()];
        let attestation_time = 0; // Placeholder for attestation time
        (pub_keys, pub_nonces, attestation_time)
    }

    fn get_attestation(&self, _event_id: u32) -> (Self::Outcome, Self::Attestation) {
        (self.outcome, self.compute_attestation())
    }
}
