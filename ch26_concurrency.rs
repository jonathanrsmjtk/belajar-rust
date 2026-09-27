/*
In Concurrent programming, different parts of a program execute independently. 
On the other hand, in parallel programming, different parts of a program execute at the same time. 
Both the models are equally important as more computers take advantage of their multiple processors.
*/

use std::thread;
use std::time::Duration;

fn main() {
    // create new thread
    thread::spawn( || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // executed by main thread
    for i in 1..10 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}