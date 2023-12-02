use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PublicKey};

use rand::rngs::ThreadRng;
use schnorr_fun::{
    adaptor::{Adaptor, EncryptedSign},
    fun::{marker::*, nonce, Scalar},
    Message, Schnorr,
};
use sha2::Sha256;

fn main() {
    byte_channel_point_experiment();
}

fn demo() {
    #![allow(dead_code)]
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);
    let signing_keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    let verification_key = signing_keypair.public_key();
    // Oracle Y_i, and y_i
    let decryption_key = Scalar::random(&mut rand::thread_rng());
    let encryption_key = schnorr.encryption_key_for(&decryption_key);
    let message = Message::<Public>::plain("text-bitcoin", b"send 1 BTC to Bob");

    // Alice knows: signing_keypair, encryption_key
    // Bob knows: decryption_key, verification_key

    // Alice creates an encrypted signature and sends it to Bob
    let encrypted_signature = schnorr.encrypted_sign(&signing_keypair, &encryption_key, message);

    // Bob verifies it and decrypts it
    assert!(schnorr.verify_encrypted_signature(&verification_key, &encryption_key, message, &encrypted_signature));
    let signature = schnorr.decrypt_signature(decryption_key, encrypted_signature.clone());

    // Bob then broadcasts the signature to the public.
    // Once Alice sees it she can recover Bob's secret decryption key
    match schnorr.recover_decryption_key(&encryption_key, &encrypted_signature, &signature) {
        Some(decryption_key) => println!("Alice got the decryption key {}", decryption_key),
        None => eprintln!("signature is not the decryption of our original encrypted signature"),
    }
}

// Function to generate a random key pair, compressed public key, and Bitcoin address
fn rand_pair_gen() {
    #![allow(dead_code)]
    // Generate random key pair.
    let s = Secp256k1::new();
    let public_key = s.generate_keypair(&mut rand::thread_rng()).1;
    let compressed_public_key = PublicKey::new(public_key);

    // Generate pay-to-pubkey-hash address.
    let address = Address::p2pkh(&compressed_public_key, Network::Bitcoin);

    println!(
        "This is (uncompressed) public key: {:?}\n",
        public_key.serialize_uncompressed()
    );
    println!("This is compressed public key: {}", compressed_public_key);
    println!("This is bitcoin address: {}", address);
}

use std::sync::mpsc;
use std::thread;

fn byte_channel_string_experiment() {
    // Create a channel for sending bytes
    let (tx, rx) = mpsc::channel();

    // Spawn a secondary thread
    let handle = thread::spawn(move || {
        // Receive bytes from the main thread
        let received_bytes: [u8; 32] = rx.recv().unwrap();

        // Convert bytes back to a string
        let received_string = String::from_utf8_lossy(&received_bytes);

        // Print the reconstructed string
        println!("Hey, I got '{}' string from the main thread", received_string);
    });

    // String to be sent from the main thread
    let message = String::from("Hello from the main thread!");

    // Convert the string to bytes
    let vec_bytes = message.into_bytes();

    // Initialize array of bytes with zeros
    let mut array_bytes: [u8; 32] = [0; 32];
    let len = vec_bytes.len().min(array_bytes.len());
    array_bytes[..len].copy_from_slice(&vec_bytes[..len]);

    // Send bytes through the channel
    tx.send(array_bytes).unwrap();

    // Wait for the secondary thread to finish
    handle.join().unwrap();
}

use secp256kfun::Point;

fn byte_channel_point_experiment() {
    // Create a channel for sending bytes
    let (tx, rx) = mpsc::channel();

    // Spawn a secondary thread
    let handle = thread::spawn(move || {
        // Receive bytes from the main thread
        let received_bytes: [u8; 32] = rx.recv().unwrap();

        // Convert bytes back to a statement
        let received_point = Scalar::<Secret, _>::from_bytes(received_bytes).unwrap();

        // Print the reconstructed string
        println!("Hey, I got '{}' string from the main thread", received_point);
    });

    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Create statement/witness pair for Alice situation
    let y_a = Scalar::random(&mut rand::thread_rng());
    let Y_a = schnorr.encryption_key_for(&y_a);

    // Convert statement to bytes
    let arr_bytes: [u8; 32] = y_a.to_bytes();

    // Send bytes through the channel
    tx.send(arr_bytes).unwrap();

    // Wait for the secondary thread to finish
    handle.join().unwrap();
}
