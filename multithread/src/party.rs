use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use bitcoin::secp256k1::rand;
use rand::rngs::ThreadRng;
use schnorr_fun::{
    adaptor::{Adaptor, EncryptedSign},
    fun::{marker::*, nonce, Scalar},
    Message, Schnorr,
};
use sha2::Sha256;
use thread_broadcaster::{BroadcastListener, Controller};

pub fn party_function(
    other_party: String,
    oracle_listener: BroadcastListener<String>,
    other_party_trsm: Sender<String>,
    other_party_recv: Receiver<String>,
    blockchain_channel: Sender<String>,
) {
    // Generate nonce and schnorr signing system
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Generate signing key and verification key
    let signing_keypair = schnorr.new_keypair(Scalar::random(&mut rand::thread_rng()));
    let signing_key = signing_keypair.secret_key();
    let verification_key = signing_keypair.public_key();

    // Create transaction message to be pre-signed and sent to other party
    let byte_message = format!("Send 1 BTC to {}", other_party);
    let tx_message = Message::<Public>::plain("text-bitcoin", byte_message.as_bytes());

    // Receive Y_a and Y_b from oracle
    let Y_a = oracle_listener.channel.recv().unwrap();
    let Y_b = oracle_listener.channel.recv().unwrap();
}
