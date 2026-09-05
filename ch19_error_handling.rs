/*
Dua macam error:
1. Recoverable (result enum) : error yang tidak menghentikan proses runtime, contoh: file tidak ketemu.
2. Unrecoverable (panic) : error yang bisa menghentikan runtime, contoh: akses elemen yang tidak ada.
*/

fn main() {
/*
    Perintah setelah panic!() tidak akan dijalankan
    panic!("Hello!");
    println!("Hello world");
    Output:
        thread 'main' (7025700) panicked at ch19_error_handling.rs:8:5:
        Hello!
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
/* Contoh panic tanpa kita define secara eksplisit

    let a = [10, 20, 30];
    a[10];  

    Compiler biasanya sudah mencegah ini jadi tidak bisa dicompile.
*/
    let no = 13;
    if no % 2 == 0 {
        println!("It's even")
    }
    else {
        panic!("NOT_AN_EVEN");
    }
    println!("End of main");

    /*
    Hasil:
        thread 'main' (7044708) panicked at ch19_error_handling.rs:29:9:
        NOT_AN_EVEN
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */

}