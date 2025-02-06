use secp256k1_zkp::{Message, PublicKey, SecretKey, SECP256K1};
use sha2::{Digest, Sha256};

use crate::common::{types, ContractDescriptor, Outcome, OutcomeU32};
use crate::crypto_utils::simple_crypto_utils::SimpleCryptoUtils;
use crate::dlc_computation::simple_dlc_computation::SimpleDlcComputation;
use crate::dlc_computation::DlcComputation;
use crate::dlc_storage::simple_array_storage::SimpleArrayStorage;
use crate::dlc_storage::DlcStorage;
use crate::oracle::{Oracle, OracleAttestation};
use crate::{adaptor_signature_scheme::AdaptorSignatureScheme, dlc_controller::DlcController};
use std::io::Error;

use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;

pub struct VerySimpleController<ASigS, O>
where
    ASigS: AdaptorSignatureScheme,
    O: Oracle,
{
    name: String,
    oracle: Arc<O>,
    private_key: SecretKey,
    storage: SimpleArrayStorage<ASigS>,
    cp_verification_key: types::VerificationKey,
    cp_adaptors: Vec<ASigS::AdaptorSignature>,
    oracle_attestation: OracleAttestation,
    next_attestation_time: u32,
    _phantom_scheme: PhantomData<ASigS>,
}

impl<ASigS, O> DlcController<ASigS, O> for VerySimpleController<ASigS, O>
where
    ASigS: AdaptorSignatureScheme<Signature = secp256k1_zkp::ecdsa::Signature>,
    O: Oracle,
{
    fn new(name: &str, oracle: Arc<O>) -> Self {
        let private_key = SecretKey::new(&mut secp256k1_zkp::rand::thread_rng());
        let storage = SimpleArrayStorage::new(256);
        let cp_verification_key =
            SecretKey::from_str("0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap()
                .public_key(SECP256K1);
        let cp_adaptors = Vec::new();
        let oracle_attestation = OracleAttestation {
            outcome: OutcomeU32::from(1024),
            attestation: SecretKey::new(&mut secp256k1_zkp::rand::thread_rng()),
        };
        let next_attestation_time = 0;

        Self {
            name: name.to_string(),
            oracle,
            private_key,
            storage,
            cp_verification_key,
            cp_adaptors,
            oracle_attestation,
            next_attestation_time,
            _phantom_scheme: PhantomData,
        }
    }

    fn load_input(&self, _input_path: &str) -> Result<(), Error> {
        Ok(()) // TODO: Implement this
    }

    fn init_storage(&mut self) -> Result<(), Error> {
        let cd: ContractDescriptor<OutcomeU32> =
            (0..=255).map(|i| (OutcomeU32::from(i), i)).collect();

        let event_ancmt = self.oracle.get_event_announcement(0);

        let storage_elements_vec =
            SimpleDlcComputation::<ASigS, SimpleCryptoUtils>::compute_storage_elements_vec(
                &cd,
                255,
                &self.private_key,
                &event_ancmt.public_key,
                &event_ancmt.public_nonces[0],
            );

        for ((outcome, _), element) in cd.into_iter().zip(storage_elements_vec) {
            self.storage.put_element(&outcome, element)?;
        }
        Ok(())
    }

    fn share_verification_key(&self) -> types::PublicKey {
        self.private_key.public_key(secp256k1_zkp::SECP256K1)
    }

    fn share_adaptors(&self) -> Vec<ASigS::AdaptorSignature> {
        self.storage.get_all_my_adaptors()
    }

    fn save_cp_verification_key(&mut self, cp_verification_key: types::VerificationKey) {
        self.cp_verification_key = cp_verification_key;
    }

    fn save_cp_adaptors(&mut self, cp_adaptors: Vec<ASigS::AdaptorSignature>) {
        self.cp_adaptors = cp_adaptors;
    }

    fn verify_cp_adaptors(&self) -> bool {
        SimpleDlcComputation::<ASigS, SimpleCryptoUtils>::verify_cp_adaptors(
            &self.cp_verification_key,
            &self.cp_adaptors,
            self.storage.get_all_elements_vec_ref(),
        )
    }

    fn update_cp_adaptors(&mut self) -> Result<(), Error> {
        self.storage.update_cp_adaptors(self.cp_adaptors.clone())
    }

    fn wait_attestation(&mut self) -> bool {
        let mut attestation = self.oracle.get_event_attestation(0);
        attestation.outcome = OutcomeU32::from(attestation.outcome.get_value() % 256);

        self.oracle_attestation = attestation;

        if (self.name == "Alice" && self.oracle_attestation.outcome.get_value() < 128)
            || (self.name == "Bob" && self.oracle_attestation.outcome.get_value() >= 128)
        {
            true
        } else {
            println!(
                "{} did not win in DLC. It doesn't broadcast anything.",
                self.name
            );
            false
        }
    }

    fn finalize_tx(&self) -> types::FinalizedTx<ASigS::Signature> {
        let outcome_element = self
            .storage
            .get_element(&self.oracle_attestation.outcome)
            .unwrap();

        let hash = Sha256::digest(outcome_element.cet.as_bytes());
        let hashed_message: [u8; 32] = hash.into();
        let msg = Message::from_digest_slice(&hashed_message).unwrap();

        let my_sig = self.private_key.sign_ecdsa(msg);
        let cp_sig = ASigS::adapt(
            &outcome_element.cp_adaptor_signature.unwrap(),
            &self.oracle_attestation.attestation,
        );

        if self.name == "Alice" {
            types::FinalizedTx::<ASigS::Signature>::new(outcome_element.cet, my_sig, cp_sig)
        } else if self.name == "Bob" {
            types::FinalizedTx::<ASigS::Signature>::new(outcome_element.cet, cp_sig, my_sig)
        } else {
            // Fallback (or panic) if name is neither "Alice" nor "Bob"
            panic!("Unknown controller name: {}", self.name);
        }
    }

    // fn broadcast_to_blockchain(self) -> Result<(), Error> {
    //     // ...placeholder...
    //     unimplemented!()
    // }
}
