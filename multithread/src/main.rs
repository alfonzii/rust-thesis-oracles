mod oracle;
mod user;

use crossbeam_channel::Sender;
use secp256kfun::{marker::*, Point, Scalar};
use std::{sync::mpsc, thread};
use thread_broadcaster::{BroadcastListener, Broadcaster};

fn main() {
    let (b, s1) = Broadcaster::<[u8; 33]>::new();
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
        //thread_body2(s1);
    });
    thread::spawn(move || {
        //thread_body2(s2);
        user::user_main(String::from("Bob"), ls2, tx_bob, rx_bob);
    });

    oracle_thread.join().unwrap();
}

fn get_from_bigger_arr(source: &[u8], destination: &mut [u8]) {
    for i in 0..destination.len().min(source.len()) {
        destination[i] = source[i];
    }
}

fn thread_body(sender: Sender<Sender<[u8; 33]>>) {
    #![allow(dead_code)]
    let ls = BroadcastListener::register_broadcast_listener(sender);
    let mut i = 0;
    for byte_msg in ls.channel {
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
}

fn thread_body2(sender: Sender<Sender<[u8; 33]>>) {
    #![allow(dead_code)]
    let ls = BroadcastListener::register_broadcast_listener(sender);
    let byte_statement_a = ls.channel.recv().unwrap();
    let byte_statement_b = ls.channel.recv().unwrap();

    let statement_a = Point::<_, Public, NonZero>::from_bytes(byte_statement_a).unwrap();
    let statement_b = Point::<_, Public, NonZero>::from_bytes(byte_statement_b).unwrap();
    println!(
        "got statement_a broadcast: {} on thread {:?}",
        statement_a,
        thread::current().id()
    );
    println!(
        "got statement_b broadcast: {} on thread {:?}",
        statement_b,
        thread::current().id()
    );

    let msg_witness = ls.channel.recv().unwrap();
    let mut witness_byte_arr: [u8; 32] = [0; 32];
    get_from_bigger_arr(&msg_witness, &mut witness_byte_arr);
    let witness_scalar = Scalar::<Secret, _>::from_bytes(witness_byte_arr).unwrap();
    println!(
        "got witness broadcast: {} on thread {:?}",
        witness_scalar,
        thread::current().id()
    );
}
