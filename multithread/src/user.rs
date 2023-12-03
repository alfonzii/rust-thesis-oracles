// Standard library imports
use std::sync::mpsc::{Receiver, Sender};

// External imports
use bitcoin::secp256k1::rand;
use rand::rngs::ThreadRng;
use schnorr_fun::adaptor::EncryptedSignature;
use schnorr_fun::{
    adaptor::{Adaptor, EncryptedSign},
    fun::{marker::*, nonce, Scalar},
    Message, Schnorr,
};
use secp256kfun::Point;
use sha2::Sha256;
use thread_broadcaster::BroadcastListener;

// Custom imports
use crate::oracle::OracleBroadcastType;

pub fn user_main(
    my_name: String,
    oracle_listener: BroadcastListener<OracleBroadcastType>,
    other_user_trsm: Sender<EncryptedSignature>,
    other_user_recv: Receiver<EncryptedSignature>,
    //blockchain_channel: Sender<String>,
) {
    // Generate nonce and schnorr signing system
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Generate signing key and verification key
    let signing_keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    let verification_key = signing_keypair.public_key();

    // Create other_user String
    let other_user: String = match my_name.as_str() {
        "Alice" => String::from("Bob"),
        "Bob" => String::from("Alice"),
        _ => panic!("Unexpected value in my_name"), // Panic if my_name is neither "Alice" nor "Bob"
    };

    //println!("[{:?}]: {}", thread::current().id(), my_name);

    // Create transaction message to be pre-signed and sent to other user
    let message = format!("Send 1 BTC to {}", other_user);
    let tx_message = Message::<Public>::plain("text-bitcoin", message.as_bytes());

    // Receive Y_a and Y_b from oracle
    let received_statement_a = oracle_listener.channel.recv().unwrap();
    let received_statement_b = oracle_listener.channel.recv().unwrap();
    let mut Y_a;
    let mut Y_b;

    if let OracleBroadcastType::Statement(statement_a) = received_statement_a {
        Y_a = statement_a;
    } else {
        panic!("Unexpected value in received_statement_a");
    }
    if let OracleBroadcastType::Statement(statement_b) = received_statement_b {
        Y_b = statement_b;
    } else {
        panic!("Unexpected value in received_statement_b");
    }

    // Create pre-signature
    let presignature: EncryptedSignature = if my_name == "Alice" {
        schnorr.encrypted_sign(&signing_keypair, &Y_b, tx_message)
    } else if my_name == "Bob" {
        schnorr.encrypted_sign(&signing_keypair, &Y_a, tx_message)
    } else {
        panic!("Unexpected value in my_name")
    };

    // Send pre-signature to other user through channel
    other_user_trsm.send(presignature.clone()).unwrap();

    // Wait for his pre-signature as well
    let other_presignature = other_user_recv.recv().unwrap();

    // TODO: Verify other user pre-signature
    // We dont have this feature implemented here yet, because it's not urgently needed for prototype showcase purposes
    // Our users can just trust, that they sent each other valid pre-signature, because we are actually doing it.
    // However, for good practice and rigorous way, verification should be implemented in future.
    // Reason why we didn't implement it here now, is because of time consuming, because we would need to solve
    // either serialization problem of two different types sent through one canal, or to use some other way
    // to let the other party know our verification key.

    // assert!(schnorr.verify_encrypted_signature(&verification_key, &encryption_key, message, &encrypted_signature));

    // Listen to oracle and wait for witness broadcast
    let msg_witness = oracle_listener.channel.recv().unwrap();
    let mut witness_scalar;
    if let OracleBroadcastType::Witness(witness) = msg_witness {
        witness_scalar = witness;
    } else {
        panic!("Unexpected value in msg_witness");
    }

    // Adapt pre-signature
    let signature = schnorr.decrypt_signature(witness_scalar.clone(), other_presignature.clone());

    //println!("Witness scalar I got is: {}", witness_scalar.clone());

    // Once we have signature, we can make judgement if we won the bet
    if my_name == "Alice" {
        match schnorr.recover_decryption_key(&Y_a, &other_presignature, &signature) {
            Some(some_witness_scalar) => println!(
                "[{}]: Me, Alice, won the bet and get 1 BTC thanks to witness: {}",
                my_name, some_witness_scalar
            ),
            None => {
                eprintln!(
                    "[{}]: signature is not the decryption of our original encrypted signature",
                    my_name
                )
            }
        }
    } else if my_name == "Bob" {
        match schnorr.recover_decryption_key(&Y_b, &other_presignature, &signature) {
            Some(some_witness_scalar) => println!(
                "[{}]: Me, Bob, won the bet and get 1 BTC thanks to witness: {}",
                my_name, some_witness_scalar
            ),
            None => {
                eprintln!(
                    "[{}]: signature is not the decryption of our original encrypted signature",
                    my_name
                )
            }
        }
    } else {
        panic!("Unexpected value in my_name");
    }
}
