// threads.rs

use std::thread;
use std::time::Duration;

fn main() {
    // this thread is stopped whem the main thread finishes execution
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        // The calls to thread::sleep force a thread to stop its execution for a short
        // duration, allowing a different thread to run. The threads will probably take
        // turns, but that isn’t guaranteed: it depends on how your operating system
        // schedules the threads.
        thread::sleep(Duration::from_millis(1));
    }
}
