mod oracle;
mod party;

use std::{sync::mpsc, thread};
use thread_broadcaster::{BroadcastListener, Broadcaster};

fn main() {
    let (b, s1) = Broadcaster::<String>::new();
    let s2 = s1.clone();

    let oracle_thread = thread::spawn(move || {
        oracle::oracle_main(b);
    });

    thread::spawn(move || {
        let ls1 = BroadcastListener::register_broadcast_listener(s1);
        for msg in ls1.channel {
            println!(
                "got broadcast with data: {} on thread {:?}",
                msg,
                thread::current().id()
            );
        }
    });
    thread::spawn(move || {
        let ls2 = BroadcastListener::register_broadcast_listener(s2);
        for msg in ls2.channel {
            println!(
                "got broadcast with data: {} on thread {:?}",
                msg,
                thread::current().id()
            );
        }
    });

    oracle_thread.join().unwrap();
}
