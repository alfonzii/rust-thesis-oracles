/*use std::thread;
use std::time::Duration;

use crossbeam_channel;

fn main() {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let receiver2 = receiver.clone();


    thread::spawn(move || {
        for rx in receiver {
            println!("[T1] Got: {}", rx);
        }
    });

    thread::spawn(move || {
        for rx in receiver2 {
            println!("[T2] Got: {}", rx);
        }
    });

    let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("mainthread\n"),
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

    for val in vals {
        sender.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

*/

mod oracle;
mod party;


//use core::time;
use std::{thread, sync::mpsc};
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

/* 
mod oracle;

use std::thread;

fn main() {
    println!("Main: Hello from main!");

    // Spawn a thread from the my_module module
    let my_thread = thread::spawn(|| {
        oracle::my_thread_function();
    });

    // Join the thread to ensure it finishes before the main thread exits
    my_thread.join().unwrap();

    println!("Main: Thread is done!");
}
*/