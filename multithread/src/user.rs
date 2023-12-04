// Standard library imports
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

// External imports
use bitcoin::secp256k1::rand;
use rand::rngs::ThreadRng;
use schnorr_fun::adaptor::EncryptedSignature;
use schnorr_fun::Signature;
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

// Pre-signature and verification key pair
type PresignatureVerifKeyPair = (EncryptedSignature, Point<EvenY>);
// Public key, message and signature tuple
pub type SignatureSchemeTuple = (Point<EvenY>, String, Signature);

pub fn user_main(
    my_name: String,
    oracle_listener: BroadcastListener<OracleBroadcastType>,
    other_user_trsm: Sender<PresignatureVerifKeyPair>,
    other_user_recv: Receiver<PresignatureVerifKeyPair>,
    blockchain_channel_trsm: Sender<SignatureSchemeTuple>,
) {
    println!("[{}]: {:?}", my_name, thread::current().id());

    // Generate nonce and schnorr signing system
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Generate signing key and verification key
    let signing_keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    let my_verification_key = signing_keypair.public_key();

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

    #[allow(non_snake_case)]
    let Y_a;
    #[allow(non_snake_case)]
    let Y_b;

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

    // Send pre-signature and verification key to other user through channel
    other_user_trsm.send((presignature.clone(), my_verification_key)).unwrap();

    // Wait for his pre-signature and verification key as well
    let (other_presignature, other_verification_key) = other_user_recv.recv().unwrap();

    // Verify other user pre-signature
    match my_name.as_str() {
        "Alice" => {
            assert!(schnorr.verify_encrypted_signature(&other_verification_key, &Y_a, Message::<Public>::plain("text-bitcoin", b"Send 1 BTC to Alice"), &other_presignature));
            println!("[Alice]: I verified Bob's pre-signature successfully");
        }
        "Bob" => {
            assert!(schnorr.verify_encrypted_signature(&other_verification_key, &Y_b, Message::<Public>::plain("text-bitcoin", b"Send 1 BTC to Bob"), &other_presignature));
            println!("[Bob]: I verified Alice's pre-signature successfully");
        }
        _ => panic!("Unexpected value in my_name"), // Panic if my_name is neither "Alice" nor "Bob"
    }

    // Listen to oracle and wait for witness broadcast
    let msg_witness = oracle_listener.channel.recv().unwrap();
    let witness_scalar;
    if let OracleBroadcastType::Witness(witness) = msg_witness {
        witness_scalar = witness;
    } else {
        panic!("Unexpected value in msg_witness");
    }

    // Adapt pre-signature
    let other_signature = schnorr.decrypt_signature(witness_scalar.clone(), other_presignature.clone());

    // Once we have signature, we can make judgement if we won the bet
    if my_name == "Alice" {
        match schnorr.recover_decryption_key(&Y_a, &other_presignature, &other_signature) {
            Some(some_witness_scalar) => println!("[Alice]: Me, Alice, won the bet and get 1 BTC thanks to witness: {}", some_witness_scalar),
            None => {
                eprintln!("[Alice]: signature is not the decryption of our original encrypted signature")
            }
        }
    } else if my_name == "Bob" {
        match schnorr.recover_decryption_key(&Y_b, &other_presignature, &other_signature) {
            Some(some_witness_scalar) => println!("[Bob]: Me, Bob, won the bet and get 1 BTC thanks to witness: {}", some_witness_scalar),
            None => {
                eprintln!("[Bob]: signature is not the decryption of our original encrypted signature")
            }
        }
    } else {
        panic!("Unexpected value in my_name");
    }

    // Create transaction message to be used in signature verification
    let btc_to_me_message = format!("Send 1 BTC to {}", my_name);

    // Send signature scheme to blockchain to be verified and accepted (if valid)
    blockchain_channel_trsm.send((other_verification_key, btc_to_me_message, other_signature)).unwrap();
}
