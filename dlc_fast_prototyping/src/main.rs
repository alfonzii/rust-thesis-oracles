// main.rs

use std::{sync::Arc, time::Instant};

use adaptor_signature_scheme::EcdsaAdaptorSignatureScheme;
use common::{
    constants::{ALICE, BOB, MAX_OUTCOME},
    runparams::{MyAdaptorSignatureScheme, MyCryptoUtils, MyOracle, MySignature},
    types, FinalizedTx,
};
use dlc_controller::{very_simple_controller::VerySimpleController, DlcController};
use secp256k1_zkp::Secp256k1;

mod adaptor_signature_scheme;
mod common;
mod crypto_utils;
mod dlc_computation;
mod dlc_controller;
mod dlc_storage;
mod oracle;
mod parser;

// TODO: spisat dakde ze co s cim jak suvisi a interaguje (v ramci tych modulov/typov), ze napr. CryptoUtils musi byt rovnaky na strane clienta a Oracle
// alebo trebarz ze DlcComputation a DlcStorage musia byt specificke pre Controller, tak budu napr. v jeho implementaciii a nemozeme menit ich, iba cely DlcController... atd

mod bench {
    use std::time::{Duration, Instant};

    #[cfg(feature = "enable_benchmarks")]
    pub fn measure_step<R, F: FnOnce() -> R>(
        label: &str,
        steps: &mut Vec<(String, Duration)>,
        f: F,
    ) -> R {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        println!("{}: {}ms", label, duration.as_millis());
        steps.push((label.to_string(), duration));
        result
    }

    #[cfg(not(feature = "enable_benchmarks"))]
    pub fn measure_step<R, F: FnOnce() -> R>(
        _label: &str,
        _steps: &mut Vec<(String, Duration)>,
        f: F,
    ) -> R {
        // No-op version
        f()
    }

    // Function to print benchmarking table.
    pub fn print_table(steps: &Vec<(String, Duration)>, total_time: Duration) {
        println!("\n-------------------------------------------------------------");
        println!("{:<35}{:<15}{:<15}", "STEP", "TIME", "RATIO");
        println!("-------------------------------------------------------------");
        for (label, step_dur) in steps {
            let ratio = (step_dur.as_secs_f64() / total_time.as_secs_f64()) * 100.0;
            println!(
                "{:<35}{:<15}{:.2}%",
                label,
                format!("{}ms", step_dur.as_millis()),
                ratio
            );
        }
        println!("-------------------------------------------------------------");
        println!(
            "{:<35}{:<15}{}",
            "TOTAL RUNTIME:",
            format!("{}ms", total_time.as_millis()),
            "100.00%"
        );
        println!("-------------------------------------------------------------");
    }
}

// Validates (locally) final transaction which would be broadcasted to blockchain and by doing so it
// simulates blockchain acceptance or rejection of final tx.
fn finalized_tx_valid(
    finalized_tx: &FinalizedTx<MySignature>,
    multisig: &types::MultisigFundAddress,
) -> bool {
    let secp = Secp256k1::verification_only();

    let msg = match common::fun::create_message(finalized_tx.payload.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let (sig1_ok, sig2_ok) = {
        #[cfg(feature = "ecdsa")]
        {
            let sig1_ok = secp
                .verify_ecdsa(&msg, &finalized_tx.signature1, &multisig.public_key1)
                .is_ok();
            let sig2_ok = secp
                .verify_ecdsa(&msg, &finalized_tx.signature2, &multisig.public_key2)
                .is_ok();
            (sig1_ok, sig2_ok)
        }
        #[cfg(feature = "schnorr")]
        {
            let sig1_ok = secp
                .verify_schnorr(
                    &finalized_tx.signature1,
                    &msg,
                    &multisig.public_key1.x_only_public_key().0,
                )
                .is_ok();
            let sig2_ok = secp
                .verify_schnorr(
                    &finalized_tx.signature2,
                    &msg,
                    &multisig.public_key2.x_only_public_key().0,
                )
                .is_ok();
            (sig1_ok, sig2_ok)
        }
    };

    if sig1_ok && sig2_ok {
        println!("Transaction \"{}\" is valid.", finalized_tx.payload);
        return true;
    }
    false
}

fn main() {
    let start = Instant::now();
    let mut steps = Vec::new();

    // Create oracle pointer, so both controllers use API of same oracle
    let oracle = Arc::new(MyOracle::new());
    println!(
        "Oracle outcome: {:?} from {:?}",
        oracle.get_outcome() % MAX_OUTCOME,
        MAX_OUTCOME
    );

    // Create controllers
    let mut controller_alice =
        bench::measure_step("Construct controller (Alice)", &mut steps, || {
            VerySimpleController::<MyAdaptorSignatureScheme, MyCryptoUtils, MyOracle>::new(
                ALICE,
                Arc::clone(&oracle),
            )
        });
    let mut controller_bob = bench::measure_step("Construct controller (Bob)", &mut steps, || {
        VerySimpleController::<MyAdaptorSignatureScheme, MyCryptoUtils, MyOracle>::new(
            BOB,
            Arc::clone(&oracle),
        )
    });

    // Load input files
    bench::measure_step("Load input (Alice)", &mut steps, || {
        controller_alice
            .load_input("numerical_contract_input.json")
            .unwrap();
    });
    bench::measure_step("Load input (Bob)", &mut steps, || {
        controller_bob
            .load_input("numerical_contract_input.json")
            .unwrap();
    });

    // Initialize storage
    bench::measure_step("Init storage (Alice)", &mut steps, || {
        controller_alice.init_storage().unwrap();
    });
    bench::measure_step("Init storage (Bob)", &mut steps, || {
        controller_bob.init_storage().unwrap();
    });

    // Share verification keys and adaptors
    bench::measure_step("Exchange keys and adaptors (Alice)", &mut steps, || {
        controller_alice.save_cp_verification_key(controller_bob.share_verification_key());
        controller_alice.save_cp_adaptors(controller_bob.share_adaptors());
    });
    bench::measure_step("Exchange keys and adaptors (Bob)", &mut steps, || {
        controller_bob.save_cp_verification_key(controller_alice.share_verification_key());
        controller_bob.save_cp_adaptors(controller_alice.share_adaptors());
    });

    // Verify counterparty adaptors
    bench::measure_step("Verify adaptors (Alice)", &mut steps, || {
        assert!(
            controller_alice.verify_cp_adaptors(),
            "Counterparty adaptors are not valid."
        );
    });
    bench::measure_step("Verify adaptors (Bob)", &mut steps, || {
        assert!(
            controller_bob.verify_cp_adaptors(),
            "Counterparty adaptors are not valid."
        );
    });

    // Update counterparty adaptors
    bench::measure_step("Update cp adaptors (Alice)", &mut steps, || {
        controller_alice.update_cp_adaptors().unwrap()
    });
    bench::measure_step("Update cp adaptors (Bob)", &mut steps, || {
        controller_bob.update_cp_adaptors().unwrap()
    });

    // Fund the multisig address
    let multisig = types::MultisigFundAddress::new(
        controller_alice.share_verification_key(),
        controller_bob.share_verification_key(),
    );

    // Wait for oracle attestation and finalize if positive
    bench::measure_step("Wait attestation + finalize (Alice)", &mut steps, || {
        if controller_alice.wait_attestation() {
            let finalized_tx = controller_alice.finalize_tx();
            assert!(finalized_tx_valid(&finalized_tx, &multisig));
        }
    });
    bench::measure_step("Wait attestation + finalize (Bob)", &mut steps, || {
        if controller_bob.wait_attestation() {
            let finalized_tx = controller_bob.finalize_tx();
            assert!(finalized_tx_valid(&finalized_tx, &multisig));
        }
    });

    #[cfg(feature = "enable_benchmarks")]
    let total_time = start.elapsed();
    #[cfg(feature = "enable_benchmarks")]
    bench::print_table(&steps, total_time);
}

#[cfg(test)]
mod tests {
    use crate::{
        adaptor_signature_scheme::AdaptorSignatureScheme,
        crypto_utils::simple_crypto_utils::SimpleCryptoUtils,
    };

    use super::*;
    use rand::thread_rng;
    use secp256k1_zkp::{Keypair, Message, Secp256k1};
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
        let keypair = Keypair::new(&secp, &mut rng);
        // Generate nonce keypair (for anticipation point / attestation)
        let (nonce_sk, nonce_pk) = secp.generate_keypair(&mut rng);

        // Create SimpleCryptoUtils engine
        let crypto_utils_engine = MyCryptoUtils::new(&keypair.public_key(), &nonce_pk);

        // Create message
        let message_str = "Adaptor signature test";
        let hash = Sha256::digest(message_str.as_bytes());
        let msg = Message::from_digest_slice(&hash).unwrap();

        // Create outcome
        let outcome_value = 42u32;
        let outcome = OutcomeU32::from(outcome_value);

        // Compute anticipation point using SimpleCryptoUtils
        let anticipation_point = crypto_utils_engine
            .compute_anticipation_point(&outcome)
            .expect("Failed to compute anticipation point");

        // Create adaptor signature and verify pre-adaptation
        let adaptor_sig =
            EcdsaAdaptorSignatureScheme::pre_sign(&keypair, &msg, &anticipation_point);
        assert!(
            EcdsaAdaptorSignatureScheme::pre_verify(
                &keypair.public_key(),
                &msg,
                &anticipation_point,
                &adaptor_sig
            ),
            "Pre-verification failed"
        );

        // Compute attestation using SimpleCryptoUtils (using nonce_sk as private nonce)
        let attestation = crypto_utils_engine
            .compute_attestation(&keypair.secret_key(), &nonce_sk, &outcome)
            .expect("Failed to compute attestation");

        // Adapt the adaptor signature using computed attestation and verify signature
        let adapted_sig = EcdsaAdaptorSignatureScheme::adapt(&adaptor_sig, &attestation);
        assert!(
            secp.verify_ecdsa(&msg, &adapted_sig, &keypair.public_key())
                .is_ok(),
            "Adapted signature verification failed"
        );

        println!("ECDSA adaptor signature test passed.");
    }
}
