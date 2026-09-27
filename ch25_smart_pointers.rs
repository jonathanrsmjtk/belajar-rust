/* 
Rust allocates everything on the stack by default. You can store things on the heap by wrapping them in smart pointers like Box. 
Types like Vec and String implicitly help heap allocation. Smart pointers implement traits listed in the table below. 
These traits of the smart pointers differentiate them from an ordinary struct −

1. std::ops::Deref
Used for immutable dereferencing operations, like *v.

2. std::ops::Drop
Used to run some code when a value goes out of scope. This is sometimes called a destructor
*/
fn main() {
/*
    The Box smart pointer also called a box allows you to store data on the heap rather than the stack. 
    The stack contains the pointer to the heap data. A Box does not have performance overhead, other than storing their data on the heap.
*/
    let var_i32 = 5;
    let b = Box::new(var_i32);
    println!("b = {}", b);

    let x = 5;
    let y = Box::new(x);
    println!("{}", 5==x);
    println!("{}", 5==*y); // karena variabel y point out ke heap, jadi buat akses, mesti pake *y
}