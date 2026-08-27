extern crate movies_lib;
use movies_lib::movies::play;

fn main() {
    println!("Inside main of test ");
    play("Rush Hour 3".to_string());
}