// main.rs

use adaptor_signatures::AdaptorSignatureScheme;
use schnorr_fun::{self, Message, Schnorr};
use secp256k1_zkp::SECP256K1;
use secp256kfun::{marker::Public, Point};
use secp_utils::schnorrsig_compute_anticipation_point;

use oracle::Oracle;
use sha2::{Digest, Sha256};

mod adaptor_signatures;
mod oracle;
mod secp_utils;

fn main() {
    schnorr_fun_demo();
}

fn schnorr_fun_demo() {
    let schnorr = Schnorr::<Sha256>::verify_only();

    let oracle = oracle::RandIntOracle::new();
    let public_key = oracle.get_public_key()[0];
    let public_nonce = oracle.get_announcement(0).1[0];
    let outcome = oracle.get_outcome();
    let attestation = oracle.get_attestation(0).1;

    println!("[Oracle]: Public key: {:?}", public_key);
    println!("[Oracle]: Public nonce: {:?}", public_nonce);

    let schnorr_scheme = adaptor_signatures::SchnorrFunAdaptorSignatureScheme::new();
    let keypair = schnorr_fun::fun::KeyPair::<schnorr_fun::fun::marker::EvenY>::new(
        schnorr_fun::fun::Scalar::random(&mut rand::thread_rng()),
    );
    let message = String::from("send 1 BTC to Bob");

    // Make a bet
    println!(
        "[Alice]: I, Alice, will send 1 BTC to Bob, if the outcome of oracle is {:?}",
        outcome
    );

    let msg = Message::<Public>::plain("text-bitcoin", message.as_bytes());

    let atp_point =
        schnorrsig_compute_anticipation_point(SECP256K1, &public_key, &public_nonce, outcome)
            .unwrap();

    let atp_point = Point::from_bytes(atp_point.serialize()).unwrap();

    let pre_signature = schnorr_scheme.pre_sign(&keypair, &message, &atp_point);
    let verif_key = keypair.public_key();

    // Share pre_signature and verif_key with the Bob so he can verify the adaptor signature of message/tx:
    // "send 1 BTC to Bob"
    assert_eq!(
        schnorr_scheme.pre_verify(&verif_key, &message, &atp_point, &pre_signature),
        true
    );

    println!(
        "[Bob]: Alice has sent me adaptor signature: {:?}",
        pre_signature
    );

    // (Simulate) wait for the attestation to be revealed
    println!("\n\nWaiting for oracle to attest\n\n");
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("Outcome: {:?}", outcome);
    println!("Attestation: {:?}", attestation);

    let attestation = schnorr_fun::fun::Scalar::from_bytes(attestation.secret_bytes()).unwrap();

    // Adapt the pre_signature to get the final signature
    let signature = schnorr_scheme.adapt(&pre_signature, &attestation);

    assert!(schnorr.verify(&keypair.public_key(), msg, &signature));
    println!("The adapted signature correctly signs the message.");
}

fn ecdsa_zkp_demo() {
    let oracle = oracle::RandIntOracle::new();
    let public_key = oracle.get_public_key()[0];
    let public_nonce = oracle.get_announcement(0).1[0];
    let outcome = oracle.get_outcome();
    let attestation = oracle.get_attestation(0).1;

    println!("[Oracle]: Public key: {:?}", public_key);
    println!("[Oracle]: Public nonce: {:?}", public_nonce);

    let ecdsa_scheme = adaptor_signatures::EcdsaZkpAdaptorSignatureScheme::new();
    let (signing_key, verification_key) = SECP256K1.generate_keypair(&mut rand::thread_rng());

    let message = String::from("send 1 BTC to Bob");
    let hash = Sha256::digest(message.as_bytes());
    let hashed_message: [u8; 32] = hash.into();

    // Make a bet
    println!(
        "[Alice]: I, Alice, will send 1 BTC to Bob, if the outcome of oracle is {:?}",
        outcome
    );

    let atp_point =
        schnorrsig_compute_anticipation_point(SECP256K1, &public_key, &public_nonce, outcome)
            .unwrap();

    let msg = secp256k1_zkp::Message::from_digest_slice(&hashed_message).unwrap();
    let pre_signature = ecdsa_scheme.pre_sign(&signing_key, &msg, &atp_point);

    // Share pre_signature and verif_key with the Bob so he can verify the adaptor signature of message/tx:
    // "send 1 BTC to Bob"
    assert_eq!(
        ecdsa_scheme.pre_verify(&verification_key, &msg, &atp_point, &pre_signature),
        true
    );

    println!(
        "[Bob]: Alice has sent me adaptor signature: {:?}",
        pre_signature
    );

    // (Simulate) wait for the attestation to be revealed
    println!("\n\nWaiting for oracle to attest\n\n");
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("Outcome: {:?}", outcome);
    println!("Attestation: {:?}", attestation);

    let signature = ecdsa_scheme.adapt(&pre_signature, &attestation);

    assert!(SECP256K1
        .verify_ecdsa(&msg, &signature, &verification_key)
        .is_ok());
    println!("The adapted signature correctly signs the message.");
}
