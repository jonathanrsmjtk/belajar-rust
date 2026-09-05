/*
Ownership

Memory dialokasikan dalam: 
- Stack
- Heap

Stack
A stack follows a last in first out order. Stack stores data values for which the size is known at compile time. For example, a variable of fixed size i32 is a candidate for stack allocation. Its size is known at compile time. All 
scalar types can be stored in stack as the size is fixed.
Consider an example of a string, which is assigned a value at runtime. The exact size of such a string cannot be determined at compile time. So it is not a candidate for stack allocation but for heap allocation.

Heap
The heap memory stores data values the size of which is unknown at compile time. It is used to store dynamic data. 
Simply put, a heap memory is allocated to data values that may change throughout the life cycle of the program. 
The heap is an area in the memory which is less organized when compared to stack.

Jadi bedanya, pada stack anda tahu size dan panjang data, sementara heap tidak (lebih dinamis).

Ownership -> setiap variabel adalah owner dari value.
let age = 30
maka age adalah pemilik value 30
Setiap data hanya punya satu owner di satu waktu, dan dua variable tidak bisa mengarah ke lokasi memori yang sama. Variabel pastinya akan mengarah ke lokasi memori yang berbeda.

Ownership bisa diberikan dengan:
- Memasang value dari satu variabel ke variabel lainnya
- Pass value ke sebuah fungsi
- Kembalikan value dari fungsi
*/

fn main() {
    // Pasang value dari satu variabel ke variabel lain. Ini bisa jadi memory safety karena mengontrol siapa yang bisa pakai value dan kapan perlu diperketat penggunaanya.
    /*  
        Kode ini pasti akan error. Solusi: println!(v2)

        let v = vec![1,2,3]; // v owner dari vector
        let v2 = v; // Pindah ownership ke v2
        println!("{:?}", v); // error karena v bukan owner lagi

        error[E0382]: borrow of moved value: `v`
            --> ch12_ownership.rs:34:22
                |
                32 |     let v = vec![1,2,3];
                |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
                33 |     let v2 = v;
                |              - value moved here
                34 |     println!("{:?}", v);
                |                      ^ value borrowed here after move
                |
                help: consider cloning the value if the performance cost is acceptable
                |
                33 |     let v2 = v.clone();
                |               ++++++++

        warning: unused variable: `v2`
            --> ch12_ownership.rs:33:9
                |
                33 |     let v2 = v;
                |         ^^ help: if this is intentional, prefix it with an underscore: `_v2`
                |
                = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

                error: aborting due to 1 previous error; 1 warning emitted

                For more information about this error, try `rustc --explain E0382`.
    */

    let v = vec![1,2,3]; // v owner dari vector
    let v2 = v; // Pindah ownership ke v2
    println!("{:?}", v2); // error karena v bukan owner lagi

    // Pass value ke sebuah fungsi
    /* 
        Kode ini juga pasti akan error

        let v = vec![1, 2, 3]; // vector v sebagain owner object di heap
        let v2 = v; // pindah ownership ke v2
        display(v2); // v2 pindah ke display() dan invalid
        println!("In main {:?}", v2) // v2 gak bisa dipake, jadi errornya terjadi di sini

        --> ch12_ownership.rs:70:30
            |
            68 |     let v2 = v; // pindah ownership ke v2
            |         -- move occurs because `v2` has type `Vec<i32>`, which does not implement the `Copy` trait
            69 |     display(v2); // v2 pindah ke display() dan invalid
            |             -- value moved here
            70 |     println!("In main {:?}", v2) // v2 gak bisa dipake
            |                              ^^ value borrowed here after move
            |
            note: consider changing this parameter type in function `display` to borrow instead if owning the value isn't necessary
            --> ch12_ownership.rs:73:15
            |
            73 | fn display(v: Vec<i32>) {
            |    -------    ^^^^^^^^ this parameter takes ownership of the value
            |    |
            |    in this function
            help: consider cloning the value if the performance cost is acceptable
            |
            69 |     display(v2.clone()); // v2 pindah ke display() dan invalid
            |               ++++++++

            error: aborting due to 1 previous error

            For more information about this error, try `rustc --explain E0382`.
    */

    let v = vec![1, 2, 3]; // vector v sebagain owner object di heap
    let v2 = v; // pindah ownership ke v2
    display(v2); // v2 pindah ke display() dan invalid

    // Kembalikan value dari fungsi
    let v = vec![1, 2, 3];
    let v2 = v;
    let v2_return = display_return(v2);
    println!("In main {:?}", v2_return);

    /* 
        Pada primitive type seperti f32, i32, resource yang dibutuhkan sangat kecil sehingga tidak ada konsep ownership yang terjadi.
    */
    let u1 = 10;
    let u2 = u1;

    println!("u1 = {}", u1);

}

fn display(v: Vec<i32>) {
    println!("Inside display {:?}", v);
}

fn display_return(v: Vec<i32>) -> Vec<i32> {
    println!("Inside display {:?}", v);
    v /* gak ada ini bakal error
    error[E0308]: mismatched types
    --> ch12_ownership.rs:125:35
        |
    125 | fn display_return(v: Vec<i32>) -> Vec<i32> {
        |    --------------                 ^^^^^^^^ expected `Vec<i32>`, found `()`
        |    |
        |    implicitly returns `()` as its body has no tail or `return` expression
        |
        = note: expected struct `Vec<i32>`
                found unit type `()`
    help: consider returning the local binding `v`
        |
    126 ~     println!("Inside display {:?}", v);
    127 +     v
        |

    error: aborting due to 1 previous error
    */
}