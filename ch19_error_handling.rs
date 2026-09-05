/*
Dua macam error:
1. Recoverable (result enum) : error yang tidak menghentikan proses runtime, contoh: file tidak ketemu.
2. Unrecoverable (panic) : error yang bisa menghentikan runtime, contoh: akses elemen yang tidak ada.
*/

use std::fs::File;

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
    // let no = 13;
    // if no % 2 == 0 {
    //     println!("It's even")
    // }
    // else {
    //     panic!("NOT_AN_EVEN");
    // }
    // println!("End of main");

    /*
    Hasil:
        thread 'main' (7044708) panicked at ch19_error_handling.rs:29:9:
        NOT_AN_EVEN
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */

    // Recoverable error
    let f = File::open("main.jpg");
    println!("{:?}", f);

    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        },
        Err(e) => {
            println!("file not found \n{:?}", e);
        }
    }

    // Error: Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })

    let result = is_even(13);
    match result {
        Ok(d) => {
            println!("number is even: {}", d);
        },
        Err(msg) => {
            println!("Error message is {}", msg);
        }
    }

    /* 
    unwrap(self): T
    Expects self to be Ok/Some and returns the value contained within. If it is Err or None instead, it raises a panic with the contents of the error displayed.

    expect(self, msg: &str): T
    Behaves like unwrap, except that it outputs a custom message before panicking in addition to the contents of the error.
    */
    let result = is_even(20).unwrap();
    println!("result is {}", result);

    /*
        thread 'main' (17959226) panicked at ch19_error_handling.rs:77:30:
        called `Result::unwrap()` on an `Err` value: "NOT_AN_EVEN"
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
    // let result = is_even(13).unwrap();
    // println!("result is {}", result);

    /*
    thread 'main' (17961830) panicked at ch19_error_handling.rs:85:35:
    File not able to open: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     */
    let f = File::open("pqr.txt").expect("File not able to open");
    println!("End of main");
}

fn is_even(no: i32) -> Result<bool, String> {
    if no % 2 == 0 {
        return Ok(true);
    }
    else {
        return Err("NOT_AN_EVEN".to_string());
    }
}