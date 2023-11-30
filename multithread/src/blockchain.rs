use std::thread;

pub fn blockchain_function() {
    // Your thread implementation goes here
    for i in 0..5 {
        println!("Thread: Hello from thread! Count: {}", i);
        thread::sleep(std::time::Duration::from_millis(500));
    }
}