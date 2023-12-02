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

pub fn oracle_main(broadcaster: Controller<[u8; 33]>) {
    let nonce_gen = nonce::Synthetic::<Sha256, nonce::GlobalRng<ThreadRng>>::default();
    let schnorr = Schnorr::<Sha256, _>::new(nonce_gen);

    // Create statement/witness pair for Alice situation
    let y_a = Scalar::random(&mut rand::thread_rng());
    let Y_a = schnorr.encryption_key_for(&y_a);

    // Create statement/witness pair for Bob situation
    let y_b = Scalar::random(&mut rand::thread_rng());
    let Y_b = schnorr.encryption_key_for(&y_b);

    let y_a_bytes_arr: [u8; 32] = y_a.to_bytes();
    let Y_a_bytes_arr: [u8; 33] = Y_a.to_bytes();

    let y_b_bytes_arr: [u8; 32] = y_b.to_bytes();
    let Y_b_bytes_arr: [u8; 33] = Y_b.to_bytes();

    // we wait for registration of Alice & Bob
    thread::sleep(time::Duration::from_secs(1));

    // Print messages from oracle
    println!("Y_a: {}", Y_a);
    println!("Y_b: {}\n", Y_b);

    // Broadcast statements as byte array to Alice & Bob
    broadcaster.broadcast(Y_a_bytes_arr);
    broadcaster.broadcast(Y_b_bytes_arr);

    thread::sleep(time::Duration::from_secs(1));

    // Randomly choose y_a or y_b (witness) and send it. That way we effectively choose just one winner.
    let random = rand::thread_rng().gen_range(0..100);
    let mut bigger_array: [u8; 33] = [0; 33];
    if random < 50 {
        copy_in_bigger_arr(&y_a_bytes_arr, &mut bigger_array);
        println!("\nWinning situation for Bob. Witness is y_a: {}", y_a);
    } else {
        copy_in_bigger_arr(&y_b_bytes_arr, &mut bigger_array);
        println!("\nWinning situation for Alice. Witness is y_b: {}", y_b);
    }
    broadcaster.broadcast(bigger_array);

    // we wait for listeners to pickup before being dropped
    thread::sleep(time::Duration::from_secs(1));
}

fn copy_in_bigger_arr(source: &[u8], destination: &mut [u8]) {
    for i in 0..source.len().min(destination.len()) {
        destination[i] = source[i];
    }
}
