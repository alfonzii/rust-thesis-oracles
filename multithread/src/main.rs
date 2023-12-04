mod oracle;
mod user;

use crossbeam_channel::Sender;
use oracle::OracleBroadcastType;
use secp256kfun::{marker::*, Point, Scalar};
use std::{sync::mpsc, thread};
use thread_broadcaster::{BroadcastListener, Broadcaster};

fn main() {
    let (b, s1) = Broadcaster::<OracleBroadcastType>::new();
    let s2 = s1.clone();
    let ls1 = BroadcastListener::register_broadcast_listener(s1);
    let ls2 = BroadcastListener::register_broadcast_listener(s2);

    let (tx_alice, rx_bob) = mpsc::channel();
    let (tx_bob, rx_alice) = mpsc::channel();

    let oracle_thread = thread::spawn(move || {
        oracle::oracle_main(b);
    });

    thread::spawn(move || {
        user::user_main(String::from("Alice"), ls1, tx_alice, rx_alice);
    });
    thread::spawn(move || {
        user::user_main(String::from("Bob"), ls2, tx_bob, rx_bob);
    });

    oracle_thread.join().unwrap();
}
