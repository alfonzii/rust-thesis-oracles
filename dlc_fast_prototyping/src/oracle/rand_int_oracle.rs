use crate::crypto_utils::CryptoUtils;
use crate::{
    common::{types, Outcome, OutcomeU32},
    config::NB_OUTCOMES,
};
use core::marker::PhantomData;
use secp256k1_zkp::{
    global::SECP256K1,
    rand::{thread_rng, Rng},
    Keypair, PublicKey,
};

use super::{Oracle, OracleAnnouncement, OracleAttestation};

pub struct RandIntOracle<CU: CryptoUtils> {
    nonces: Keypair,
    keys: Keypair,
    outcome: types::OutcomeU32,
    _phantom: PhantomData<CU>,
    crypto_utils_engine: CU,
}

// INFO: This oracle will always return and attest to a random integer (representing outcome) in the range [0, NB_OUTCOMES)
// so we synchronize and seamlessly work with benchmarking environment locally.
// Doing it like this we can change contracts and their respective maximum outcomes and oracle will change accordingly,
// so that it attests in correct interval.

// In real situations, we would not have constant `NB_OUTCOMES` available at oracle side, but that wouldn't matter, as oracle
// don't have to care and we would choose such oracle, that would fit our needs.

// For example,

impl<CU: CryptoUtils> RandIntOracle<CU> {
    pub fn new() -> Self {
        let nonces = Keypair::new(SECP256K1, &mut thread_rng());
        let keys = Keypair::new(SECP256K1, &mut thread_rng());

        let mut rng = thread_rng();
        let outcome = OutcomeU32::from(rng.gen::<u32>() % NB_OUTCOMES); // if we would remove "% NB_OUTCOMES", we wouldn't break any core functionality, but we would need to use input contracts with number of outcomes 2^32.

        let cu_engine = CU::new(&keys.public_key(), &nonces.public_key());

        Self {
            nonces,
            keys,
            outcome,
            _phantom: PhantomData,
            crypto_utils_engine: cu_engine,
        }
    }

    pub fn get_outcome(&self) -> u32 {
        self.outcome.get_value()
    }
}

impl<CU: CryptoUtils> Oracle for RandIntOracle<CU> {
    fn get_public_key(&self) -> PublicKey {
        self.keys.public_key()
    }

    fn get_event_announcement(&self, _event_id: u32) -> OracleAnnouncement {
        OracleAnnouncement {
            public_key: self.keys.public_key(),
            public_nonce: self.nonces.public_key(),
            _next_attestation_time: 0,
        }
    }

    /// Returns attestation structure with already moduled outcome with NB_OUTCOMES and attestation secret key
    fn get_event_attestation(&self, _event_id: u32) -> OracleAttestation {
        OracleAttestation {
            outcome: self.outcome,
            attestation: self
                .crypto_utils_engine
                .compute_attestation(
                    &self.keys.secret_key(),
                    &self.nonces.secret_key(),
                    &self.outcome,
                )
                .expect("Error computing event attestation"),
        }
    }
}
