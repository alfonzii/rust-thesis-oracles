mod blockchain;
mod oracle;
mod user;

use oracle::OracleBroadcastType;
use std::{sync::mpsc, thread};
use thread_broadcaster::{BroadcastListener, Broadcaster};

fn main() {
    let (b, s1) = Broadcaster::<OracleBroadcastType>::new();
    let s2 = s1.clone();
    let ls1 = BroadcastListener::register_broadcast_listener(s1);
    let ls2 = BroadcastListener::register_broadcast_listener(s2);

    let (tx_alice, rx_bob) = mpsc::channel();
    let (tx_bob, rx_alice) = mpsc::channel();

    let (tx_blockchain1, rx_blockchain) = mpsc::channel();
    let tx_blockchain2 = tx_blockchain1.clone();

    let oracle_thread = thread::spawn(move || {
        oracle::oracle_main(b);
    });

    let _blockchain_thread = thread::spawn(move || {
        blockchain::blockchain_main(rx_blockchain);
    });

    let alice_thread = thread::Builder::new().name("Alice_thread".into());
    let bob_thread = thread::Builder::new().name("Bob_thread".into());

    let _ = alice_thread.spawn(move || {
        user::user_main(String::from("Alice"), ls1, tx_alice, rx_alice, tx_blockchain1);
    });
    let _ = bob_thread.spawn(move || {
        user::user_main(String::from("Bob"), ls2, tx_bob, rx_bob, tx_blockchain2);
    });

    oracle_thread.join().unwrap();
}
