use secp256k1_zkp::{
    global::SECP256K1,
    rand::{thread_rng, Rng},
    Keypair,
};

use crate::common::{types, Outcome, OutcomeU32};
use crate::crypto_utils::CryptoUtils;
use crate::oracle;
use core::marker::PhantomData;

use super::{Oracle, OracleAttestation};

pub struct RandIntOracle<CU: CryptoUtils> {
    nonces: Keypair,
    keys: Keypair,
    outcome: types::OutcomeU32,
    _phantom: PhantomData<CU>,
}

impl<CU: CryptoUtils> RandIntOracle<CU> {
    pub fn new() -> Self {
        let nonces = Keypair::new(SECP256K1, &mut thread_rng());
        let keys = Keypair::new(SECP256K1, &mut thread_rng());

        let mut rng = thread_rng();
        let outcome = OutcomeU32::from(rng.gen::<u32>() % 256); // musi tu byt modulo, aby pocital atestacie len z 0-255.
                                                                // a nasledne aby ja som potom vedel adaptovat spravne adaptor-signature

        Self {
            nonces,
            keys,
            outcome,
            _phantom: PhantomData,
        }
    }

    pub fn get_outcome(&self) -> u32 {
        self.outcome.get_value()
    }
}

impl<CU: CryptoUtils> Oracle for RandIntOracle<CU> {
    fn get_public_key(&self) -> types::PublicKey {
        self.keys.public_key()
    }

    fn get_event_announcement(
        &self,
        event_id: u32,
    ) -> oracle::OracleAnnouncement<types::PublicKey, types::PublicNonce> {
        oracle::OracleAnnouncement {
            public_key: self.keys.public_key(),
            public_nonces: vec![self.nonces.public_key()],
            next_attestation_time: 0,
        }
    }

    fn get_event_attestation(&self, event_id: u32) -> OracleAttestation {
        oracle::OracleAttestation {
            outcome: self.outcome,
            attestation: CU::compute_attestation(
                &self.keys.secret_key(),
                &self.nonces.secret_key(),
                &self.outcome,
            )
            .expect("Error computing event attestation"),
        }
    }
}
