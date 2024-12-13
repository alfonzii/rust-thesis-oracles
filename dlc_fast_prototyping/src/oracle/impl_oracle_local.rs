use secp256k1_zkp::{
    global::SECP256K1,
    rand::{thread_rng, Rng},
    Keypair, PublicKey, Scalar, SecretKey,
};

use crate::oracle;
use crate::secp_utils; // Ensure the correct path to the oracle module

/// The number of digits used to represent outcome values.
const NB_DIGITS: usize = 20;

pub struct LocalOracle {
    nonces: Keypair,
    keys: Keypair,
    outcome: [u8; NB_DIGITS], // Consists of 0s and 1s
}

impl LocalOracle {
    pub fn new() -> LocalOracle {
        let nonces = Keypair::new(SECP256K1, &mut thread_rng());
        let keys = Keypair::new(SECP256K1, &mut thread_rng());

        let mut tmp_outcome = [false; NB_DIGITS];
        let mut rng = thread_rng();
        for i in 0..NB_DIGITS {
            tmp_outcome[i] = rng.gen();
        }
        let outcome = tmp_outcome.map(|b| if b { 1 } else { 0 });

        Self {
            nonces,
            keys,
            outcome,
        }
    }

    pub fn get_outcome(&self) -> &[u8; NB_DIGITS] {
        &self.outcome
    }

    fn compute_attestation(&self) -> SecretKey {
        let priv_nonce = self.nonces.secret_key();
        let priv_key = self.keys.secret_key();
        let mut seckeys: Vec<SecretKey> = Vec::new();

        for (i, &digit) in self.outcome.iter().enumerate() {
            if digit == 1 {
                let sig = secp_utils::schnorrsig_compute_oracle_attestation(
                    &SECP256K1,
                    &priv_key,
                    &priv_nonce,
                    i,
                )
                .unwrap();
                seckeys.push(sig);
            }
        }

        for i in 1..seckeys.len() {
            let sig_scalar = Scalar::from(seckeys[i]);
            seckeys[0] = seckeys[0]
                .add_tweak(&sig_scalar)
                .expect("Failed to add tweak");
        }

        seckeys[0]
    }
}

// Implement the Oracle trait for LocalOracle
impl oracle::Oracle for LocalOracle {
    type PublicKey = PublicKey;
    type PubNonce = PublicKey;
    type Outcome = [u8; NB_DIGITS];
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
