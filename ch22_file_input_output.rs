/*
Methods:
    Modules:
        1. std::fs::File
            - open(): open file in read-only mode
            - create(): opens a file in write-only mode, creates it if it doesn't exist, truncates it if it does
        2. std::fs::remove_file
            - remove_file(): removes a file from the filesystem. There is no guarantee that the file is immediately deleted.
        3. std::fs::OpenOptions
            - append(): Sets the option for the append mode of file
        4. std::io::Read
            - read_to_string(): Reads all bytes until EOF in this source, appending them to buf.
*/

use std::io::Write;
use std::fs::File;
use std::io::Read;
// use std::fs;
use std::fs::OpenOptions;

fn main() {
    let mut file = File::create("data.txt").expect("create failed");
    file.write_all("Hello world".as_bytes()).expect("write failed");
    file.write_all("\nGuyss".as_bytes()).expect("write failed");
    println!("Data has been written to file");

    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    // fs::remove_file("data.txt").expect("could not remove file");
    // println!("File has been removed");

    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot pen file");
    file.write_all("Hello guyss".as_bytes()).expect("write failed");
    file.write_all("\nbrotherrr".as_bytes()).expect("write failed");
    println!("File append success");

    // Copy a file
    let mut command_line: std::env::Args = std::env::args();
    command_line.next().unwrap();

    // skip the executable file name
    // accept the source file
    let source = command_line.next().unwrap();
    // accept the destination file
    let destination = command_line.next().unwrap();
    let mut file_in = File::open(source).unwrap();
    let mut file_out = File::create(destination).expect("create failed");
    let mut buffer = [0u8; 4096];

    loop {
        let nbytes = file_in.read(&mut buffer).unwrap();
        file_out.write(&buffer[..nbytes]).unwrap();
        if nbytes < buffer.len() {break; }
    }
}