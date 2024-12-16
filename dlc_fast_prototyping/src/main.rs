// main.rs

use adaptor_signatures::AdaptorSignatureScheme;
use rand::rngs::ThreadRng;
use schnorr_fun::{self, Message, Schnorr};
use secp256k1_zkp::SECP256K1;
use secp256kfun::{marker::Public, nonce, Point};
use secp_utils::schnorrsig_compute_anticipation_point;

use oracle::Oracle;
use sha2::Sha256;

mod adaptor_signatures;
mod oracle;
mod secp_utils;

fn main() {
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

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
    let msg_signature = schnorr.sign(&keypair, msg);
    println!(
        "[Alice private]: Signed message: {:?}, Signature: {:?}",
        message, msg_signature
    );

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
