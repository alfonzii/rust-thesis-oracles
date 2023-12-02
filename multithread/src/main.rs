mod oracle;
mod party;

use secp256kfun::{marker::*, Point, Scalar};
use std::{sync::mpsc, thread};
use thread_broadcaster::{BroadcastListener, Broadcaster};

fn main() {
    let (b, s1) = Broadcaster::<[u8; 33]>::new();
    let s2 = s1.clone();

    let oracle_thread = thread::spawn(move || {
        oracle::oracle_main(b);
    });

    thread::spawn(move || {
        let ls1 = BroadcastListener::register_broadcast_listener(s1);
        let mut i = 0;
        for byte_msg in ls1.channel {
            if i == 0 || i == 1 {
                let statement = Point::<_, Public, NonZero>::from_bytes(byte_msg).unwrap();
                println!(
                    "got statement broadcast: {} on thread {:?}",
                    statement,
                    thread::current().id()
                );
                i += 1;
            } else {
                let mut witness_byte_arr: [u8; 32] = [0; 32];
                get_from_bigger_arr(&byte_msg, &mut witness_byte_arr);
                let witness_scalar = Scalar::<Secret, _>::from_bytes(witness_byte_arr).unwrap();
                println!(
                    "got witness broadcast: {} on thread {:?}",
                    witness_scalar,
                    thread::current().id()
                );
            }
        }
    });
    thread::spawn(move || {
        let ls2 = BroadcastListener::register_broadcast_listener(s2);
        let mut i = 0;
        for byte_msg in ls2.channel {
            if i == 0 || i == 1 {
                let statement = Point::<_, Public, NonZero>::from_bytes(byte_msg).unwrap();
                println!(
                    "got statement broadcast: {} on thread {:?}",
                    statement,
                    thread::current().id()
                );
                i += 1;
            } else {
                let mut witness_byte_arr: [u8; 32] = [0; 32];
                get_from_bigger_arr(&byte_msg, &mut witness_byte_arr);
                let witness_scalar = Scalar::<Secret, _>::from_bytes(witness_byte_arr).unwrap();
                println!(
                    "got witness broadcast: {} on thread {:?}",
                    witness_scalar,
                    thread::current().id()
                );
            }
        }
    });

    oracle_thread.join().unwrap();
}

fn get_from_bigger_arr(source: &[u8], destination: &mut [u8]) {
    for i in 0..destination.len().min(source.len()) {
        destination[i] = source[i];
    }
}
