use bitcoin::secp256k1::rand::rngs::ThreadRng;
use schnorr_fun::{Message, Schnorr};
use secp256kfun::{marker::Public, nonce};
use sha2::Sha256;
use std::sync::mpsc::Receiver;
use user::SignatureSchemeTuple;

use crate::user;

pub fn blockchain_main(blockchain_channel_recv: Receiver<SignatureSchemeTuple>) {
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // This loop receives user data tuples, verifies the signature against the message with the provided public key, and logs the result.
    for received_tuple in blockchain_channel_recv {
        let (public_key, message, signature) = received_tuple;
        let byte_message = Message::<Public>::plain("text-bitcoin", message.as_bytes());
        let verification_result = schnorr.verify(&public_key, byte_message, &signature);
        match verification_result {
            true => println!("[Blockchain]: Signature verification succeeded for message: {}", message),
            false => println!("[Blockchain]: Signature verification failed for message: {}", message),
        }
    }
}
