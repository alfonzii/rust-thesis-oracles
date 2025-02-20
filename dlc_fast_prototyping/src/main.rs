// main.rs

use std::{sync::Arc, time::Instant};

use adaptor_signature_scheme::EcdsaAdaptorSignatureScheme;
use common::{constants::MAX_OUTCOME, types, FinalizedTx};
use crypto_utils::simple_crypto_utils::SimpleCryptoUtils;
use dlc_controller::{very_simple_controller::VerySimpleController, DlcController};
use oracle::RandIntOracle;
use secp256k1_zkp::Secp256k1;

mod adaptor_signature_scheme;
mod common;
mod crypto_utils;
mod dlc_computation;
mod dlc_controller;
mod dlc_storage;
mod oracle;

// TODO: spisat dakde ze co s cim jak suvisi a interaguje (v ramci tych modulov/typov), ze napr. CryptoUtils musi byt rovnaky na strane clienta a Oracle
// alebo trebarz ze DlcComputation a DlcStorage musia byt specificke pre Controller, tak budu napr. v jeho implementaciii a nemozeme menit ich, iba cely DlcController... atd
// Change following types to test different approaches to DLC
type MyCryptoUtils = SimpleCryptoUtils;
type MyAdaptorSignatureScheme = EcdsaAdaptorSignatureScheme;

type MyOracle = RandIntOracle<MyCryptoUtils>;
//type MyDlcController = .... -> spravit nejaky typp podobne ako MyOracle

// Constants
const ALICE: &str = "Alice";
const BOB: &str = "Bob";

fn main() {
    let start = Instant::now();

    // Create oracle pointer, so both controllers use API of same oracle
    let oracle = Arc::new(MyOracle::new());
    println!("Oracle outcome: {:?}", oracle.get_outcome() % MAX_OUTCOME);

    // Create controllers
    let mut controller_alice =
        VerySimpleController::<MyAdaptorSignatureScheme, _>::new(ALICE, Arc::clone(&oracle));
    let mut controller_bob =
        VerySimpleController::<MyAdaptorSignatureScheme, _>::new(BOB, Arc::clone(&oracle));

    // Load input files (does nothing now)
    controller_alice
        .load_input("some/path/to/input/file")
        .unwrap();
    controller_bob
        .load_input("some/path/to/input/file")
        .unwrap();

    // Initialize storage (heavy lifting done here)
    controller_alice.init_storage().unwrap();
    controller_bob.init_storage().unwrap();

    // Share verification keys and adaptors
    controller_alice.save_cp_verification_key(controller_bob.share_verification_key());
    controller_alice.save_cp_adaptors(controller_bob.share_adaptors());
    controller_bob.save_cp_verification_key(controller_alice.share_verification_key());
    controller_bob.save_cp_adaptors(controller_alice.share_adaptors());

    // Verify counterparty adaptors
    assert!(
        controller_alice.verify_cp_adaptors(),
        "Counterparty adaptors are not valid."
    );
    assert!(
        controller_bob.verify_cp_adaptors(),
        "Counterparty adaptors are not valid."
    );

    // Update counterparty adaptors
    controller_alice.update_cp_adaptors().unwrap();
    controller_bob.update_cp_adaptors().unwrap();

    // Fund the multisig address
    let multisig = types::MultisigFundAddress::new(
        controller_alice.share_verification_key(),
        controller_bob.share_verification_key(),
    );

    // Wait for oracle attestation and finalize if positive
    if controller_alice.wait_attestation() {
        let finalized_tx = controller_alice.finalize_tx();
        assert!(finalized_tx_valid_ecdsa(&finalized_tx, &multisig));
    }

    if controller_bob.wait_attestation() {
        let finalized_tx = controller_bob.finalize_tx();
        assert!(finalized_tx_valid_ecdsa(&finalized_tx, &multisig));
    }

    println!("Total execution time: {:?}", start.elapsed());
}

// TODO: idealne to dajak prerobit, aby to fungovalo aj na ecdsa aj na schnorr, pod jednou funkciou a nemusia byt na to 2
fn finalized_tx_valid_ecdsa(
    finalized_tx: &FinalizedTx<secp256k1_zkp::ecdsa::Signature>,
    multisig: &types::MultisigFundAddress,
) -> bool {
    let secp = Secp256k1::verification_only();

    let msg = match common::fun::create_message(finalized_tx.payload.as_bytes()) {
        Ok(msg) => msg,
        Err(_) => return false,
    };

    if secp
        .verify_ecdsa(&msg, &finalized_tx.signature1, &multisig.public_key1)
        .is_err()
    {
        return false;
    }
    if secp
        .verify_ecdsa(&msg, &finalized_tx.signature2, &multisig.public_key2)
        .is_err()
    {
        return false;
    }
    println!("Transaction \"{}\" is valid.", finalized_tx.payload);
    true
}

#[cfg(test)]
mod tests {
    use crate::adaptor_signature_scheme::AdaptorSignatureScheme;

    use super::*;
    use rand::thread_rng;
    use secp256k1_zkp::{Message, Secp256k1};
    use sha2::{Digest, Sha256};

    #[test]
    fn test_ecdsa_sign() {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut thread_rng());

        let hash = Sha256::digest("Alice gets 43 sats and Bob 120 sats".as_bytes());
        let hashed_message: [u8; 32] = hash.into();
        let msg = Message::from_digest_slice(&hashed_message).unwrap();

        let sig = secp.sign_ecdsa(&msg, &secret_key);

        assert!(secp.verify_ecdsa(&msg, &sig, &public_key).is_ok());
        println!("ECDSA test passed.");
    }

    #[test]
    fn test_ecdsa_adaptor_sign() {
        use common::types::OutcomeU32;
        use crypto_utils::CryptoUtils;

        let secp = Secp256k1::new();
        let mut rng = thread_rng();

        // Generate signer keypair
        let (signer_sk, signer_pk) = secp.generate_keypair(&mut rng);
        // Generate nonce keypair (for anticipation point / attestation)
        let (nonce_sk, nonce_pk) = secp.generate_keypair(&mut rng);

        // Create message
        let message_str = "Adaptor signature test";
        let hash = Sha256::digest(message_str.as_bytes());
        let msg = Message::from_digest_slice(&hash).unwrap();

        // Create outcome
        let outcome_value = 42u32;
        let outcome = OutcomeU32::from(outcome_value);

        // Compute anticipation point using SimpleCryptoUtils
        let anticipation_point =
            SimpleCryptoUtils::compute_anticipation_point(&signer_pk, &nonce_pk, &outcome)
                .expect("Failed to compute anticipation point");

        // Create adaptor signature and verify pre-adaptation
        let adaptor_sig =
            EcdsaAdaptorSignatureScheme::pre_sign(&signer_sk, &msg, &anticipation_point);
        assert!(
            EcdsaAdaptorSignatureScheme::pre_verify(
                &signer_pk,
                &msg,
                &anticipation_point,
                &adaptor_sig
            ),
            "Pre-verification failed"
        );

        // Compute attestation using SimpleCryptoUtils (using nonce_sk as private nonce)
        let attestation = SimpleCryptoUtils::compute_attestation(&signer_sk, &nonce_sk, &outcome)
            .expect("Failed to compute attestation");

        // Adapt the adaptor signature using computed attestation and verify signature
        let adapted_sig = EcdsaAdaptorSignatureScheme::adapt(&adaptor_sig, &attestation);
        assert!(
            secp.verify_ecdsa(&msg, &adapted_sig, &signer_pk).is_ok(),
            "Adapted signature verification failed"
        );

        println!("ECDSA adaptor signature test passed.");
    }
}
