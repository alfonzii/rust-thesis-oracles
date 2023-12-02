use core::time;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::thread;

use bitcoin::secp256k1::rand;
use schnorr_fun::{
    adaptor::Adaptor,
    fun::{nonce, Scalar},
    Schnorr,
};
use sha2::Sha256;
use thread_broadcaster::Controller;

pub fn oracle_main(broadcaster: Controller<String>) {
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Create statement/witness pair for Alice situation
    let y_a = Scalar::random(&mut rand::thread_rng());
    let Y_a = schnorr.encryption_key_for(&y_a);

    // Create statement/witness pair for Bob situation
    let y_b = Scalar::random(&mut rand::thread_rng());
    let Y_b = schnorr.encryption_key_for(&y_b);

    // we wait for registration of Alice & Bob
    thread::sleep(time::Duration::from_secs(1));

    // Broadcast statements to Alice & Bob
    let message_Ya = format!("Y_a: {}", Y_a);
    let message_Yb = format!("Y_b: {}", Y_b);
    broadcaster.broadcast(message_Ya);
    broadcaster.broadcast(message_Yb);

    thread::sleep(time::Duration::from_secs(1));

    // Randomly choose y_a or y_b (witness) and send it. That way we effectively choose just one winner.
    let random = rand::thread_rng().gen_range(0..100);
    if random < 50 {
        broadcaster.broadcast(format!("Winning situation for Bob. Witness is y_a: {}", y_a));
    } else {
        broadcaster.broadcast(format!("Winning situation for Alice. Witness is y_b: {}", y_b));
    }

    // we wait for listeners to pickup before being dropped
    thread::sleep(time::Duration::from_secs(1));
}
