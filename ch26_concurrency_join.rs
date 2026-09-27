/* 
    A spawned thread may not get a chance to run or run completely. This is because the main thread completes quickly. 
    The function spawn<F, T>(f: F) -> JoinHandlelt;T> returns a JoinHandle. The join() method on JoinHandle waits for the associated thread to finish.
*/

use std::thread;
use std::time::Duration;

fn main() {
    // create new thread
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // executed by main thread
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}