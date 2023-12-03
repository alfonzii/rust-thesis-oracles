use core::time;
use rand::rngs::ThreadRng;
use rand::Rng;
use secp256kfun::{marker::NonZero, Point};
use std::thread;

use bitcoin::secp256k1::rand;
use schnorr_fun::{
    adaptor::Adaptor,
    fun::{nonce, Scalar},
    Schnorr,
};
use sha2::Sha256;
use thread_broadcaster::Controller;

#[derive(Clone)]
pub enum OracleBroadcastType {
    Statement(Point),
    Witness(Scalar),
}

pub fn oracle_main(broadcaster: Controller<OracleBroadcastType>) {
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Create witness/statement pair for Alice situation
    let y_a: Scalar<_, NonZero> = Scalar::random(&mut rand::thread_rng());
    let Y_a = schnorr.encryption_key_for(&y_a);

    // Create witness/statement pair for Bob situation
    let y_b: Scalar<_, NonZero> = Scalar::random(&mut rand::thread_rng());
    let Y_b = schnorr.encryption_key_for(&y_b);

    // we wait for registration of Alice & Bob
    thread::sleep(time::Duration::from_secs(2));

    // Print debug messages from oracle
    println!("[Oracle]: Y_a: {}", Y_a);
    println!("[Oracle]: Y_b: {}", Y_b);
    println!("[Oracle]: y_a: {}", y_a);
    println!("[Oracle]: y_b: {}", y_b);

    // Broadcast statements to Alice & Bob
    broadcaster.broadcast(OracleBroadcastType::Statement(Y_a));
    broadcaster.broadcast(OracleBroadcastType::Statement(Y_b));

    // Here in meantime Alice and Bob exchange and verify pre-signatures
    thread::sleep(time::Duration::from_secs(2));

    // Randomly choose y_a or y_b (witness) and send it. That way we effectively choose just one winner.
    let random = rand::thread_rng().gen_range(0..100);

    let witness = if random < 50 {
        println!("\n[Oracle]: Winning situation for Alice. Witness is y_a: {}", y_a);
        y_a
    } else {
        println!("\n[Oracle]: Winning situation for Bob. Witness is y_b: {}", y_b);
        y_b
    };
    broadcaster.broadcast(OracleBroadcastType::Witness(witness));

    // we wait for listeners to pickup before being dropped
    thread::sleep(time::Duration::from_secs(2));
}
