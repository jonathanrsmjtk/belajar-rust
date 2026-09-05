use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::env::args;

fn main() {
    let mut line = String::new();
    println!("Enter your name : ");
    let b1 = stdin().read_line(&mut line).unwrap();
    println!("Hello, {}", line);
    println!("number of bytes read: {}", b1);

    let b1 = stdout().write("Hello ".as_bytes()).unwrap();
    let b2 = stdout().write(String::from("World").as_bytes()).unwrap();
    stdout().write(format!("\nbytes written {}\n", (b1+b2)).as_bytes()).unwrap();

    let cmd_line = args();
    println!("Number of elements in arguments is : {}", cmd_line.len());
    for arg in cmd_line {
        println!("[{}]", arg);
    }
    
    /* 
    Results
    ./outputs/ch21_input_output hello hi
    Enter your name : 
    Ayam
    Hello, Ayam

    number of bytes read: 5
    Hello World
    bytes written 11
    Number of elements in arguments is : 3
    [./outputs/ch21_input_output]
    [hello]
    [hi]
    */

    let cmd_line = args();
    println!("Number of elements in arguments is : {}", cmd_line.len());
    let mut sum = 0;
    let mut has_read_first_arg = false;

    for arg in cmd_line {
        if has_read_first_arg {
            sum += arg.parse::<i32>().unwrap();
        }
        has_read_first_arg = true;
    }
    println!("sum is {}", sum);

    // run with ./outputs/ch21_input_output 1 2 3 4 5
}