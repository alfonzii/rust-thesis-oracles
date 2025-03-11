use secp256k1_zkp::{Keypair, PublicKey, SecretKey, SECP256K1};

use crate::common::constants::MAX_OUTCOME;
use crate::common::runparams::{MyParser, MySignature};
use crate::common::{self, types, Outcome, OutcomeU32, ParsedContract};
use crate::crypto_utils::CryptoUtils;
use crate::dlc_computation::unified_dlc_computation::UnifiedDlcComputation;
use crate::dlc_computation::DlcComputation;
use crate::dlc_storage::simple_array_storage::SimpleArrayStorage;
use crate::dlc_storage::DlcStorage;
use crate::oracle::{Oracle, OracleAttestation};
use crate::parser::Parser;
use crate::{adaptor_signature_scheme::AdaptorSignatureScheme, dlc_controller::DlcController};

use secp256k1_zkp::rand;
use std::io::Error;

use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;
use std::u32::MAX;

// Not using those yet. Lets see in future, how will different controllers be programmed and how will
// this change. Not sure, if we actually want to allow changing of Storage and Computation for concrete controller implementations.
type MyDlcStorage<T> = SimpleArrayStorage<T>;
type MyDlcComputation<A, C> = UnifiedDlcComputation<A, C>;

pub struct VerySimpleController<ASigS, CU, O>
where
    ASigS: AdaptorSignatureScheme,
    CU: CryptoUtils,
    O: Oracle,
{
    name: String,
    oracle: Arc<O>,
    keypair: Keypair,
    storage: SimpleArrayStorage<ASigS>,
    parsed_contract: ParsedContract<OutcomeU32>,

    cp_verification_key: PublicKey,
    cp_adaptors: Vec<ASigS::AdaptorSignature>,
    oracle_attestation: OracleAttestation,
    next_attestation_time: u32,

    _phantom1: PhantomData<ASigS>,
    _phantom2: PhantomData<CU>,
}

impl<ASigS, CU, O> DlcController<ASigS, CU, O> for VerySimpleController<ASigS, CU, O>
where
    ASigS: AdaptorSignatureScheme<Signature = MySignature>,
    ASigS::AdaptorSignature: Send + Sync,
    CU: CryptoUtils + Sync,
    O: Oracle,
{
    fn new(name: &str, oracle: Arc<O>) -> Self {
        let keypair = Keypair::new(SECP256K1, &mut rand::thread_rng());
        let storage = SimpleArrayStorage::new(MAX_OUTCOME as usize);
        let parsed_contract = ParsedContract::new();
        let cp_verification_key =
            SecretKey::from_str("0000000000000000000000000000000000000000000000000000000000000001")
                .unwrap()
                .public_key(SECP256K1);
        let cp_adaptors = Vec::new();
        let oracle_attestation = OracleAttestation {
            outcome: OutcomeU32::from(MAX),
            attestation: SecretKey::new(&mut rand::thread_rng()),
        };
        let next_attestation_time = 0;

        Self {
            name: name.to_string(),
            oracle,
            keypair,
            storage,
            parsed_contract,
            cp_verification_key,
            cp_adaptors,
            oracle_attestation,
            next_attestation_time,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    fn load_input(&mut self, _input_path: &str) -> Result<(), Error> {
        self.parsed_contract = MyParser::parse_input(_input_path)?;
        Ok(())
    }

    fn init_storage(&mut self) -> Result<(), Error> {
        // Get (announcement) public key, public nonces and next attestation time from the oracle
        let event_anncmt = self.oracle.get_event_announcement(0);

        // Compute storage elements vector for all outcomes
        // create cet -> atp point -> adaptor sig -> storage element
        let storage_elements_vec = MyDlcComputation::<ASigS, CU>::compute_storage_elements_vec(
            &self.parsed_contract,
            MAX_OUTCOME.into(),
            &self.keypair,
            &event_anncmt.public_key,
            &event_anncmt.public_nonce,
        );

        // Put all elements into storage
        for ((outcome, _), element) in self.parsed_contract.iter().zip(storage_elements_vec) {
            self.storage.put_element(outcome, element)?;
        }
        Ok(())
    }

    fn share_verification_key(&self) -> PublicKey {
        self.keypair.public_key()
    }

    fn share_adaptors(&self) -> Vec<ASigS::AdaptorSignature> {
        self.storage.get_all_my_adaptors()
    }

    fn save_cp_verification_key(&mut self, cp_verification_key: PublicKey) {
        self.cp_verification_key = cp_verification_key;
    }

    fn save_cp_adaptors(&mut self, cp_adaptors: Vec<ASigS::AdaptorSignature>) {
        self.cp_adaptors = cp_adaptors;
    }

    fn verify_cp_adaptors(&self) -> bool {
        MyDlcComputation::<ASigS, CU>::verify_cp_adaptors(
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
        attestation.outcome = OutcomeU32::from(attestation.outcome.get_value() % MAX_OUTCOME);

        self.oracle_attestation = attestation;

        if (self.name == "Alice" && self.oracle_attestation.outcome.get_value() < MAX_OUTCOME / 2)
            || (self.name == "Bob"
                && self.oracle_attestation.outcome.get_value() >= MAX_OUTCOME / 2)
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

    // If we are aware of event outcome, we can finalize winning DLC transaction which will be then broadcasted to the blockchain
    fn finalize_tx(&self) -> types::FinalizedTx<ASigS::Signature> {
        let outcome_element = self
            .storage
            .get_element(&self.oracle_attestation.outcome)
            .unwrap();

        let msg = common::fun::create_message(outcome_element.cet.as_bytes()).unwrap();

        #[cfg(feature = "ecdsa")]
        let my_sig = self.keypair.secret_key().sign_ecdsa(msg);
        #[cfg(feature = "schnorr")]
        let my_sig = self.keypair.sign_schnorr(msg);

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
