fn main() {
    let my_string = "Jonathan";
    let the_float = 4.5;
    let is_the_boolean = true;

    println!("My name is {}", my_string);
    println!("The float is {}", the_float);
    println!("Is the boolean true? {}", is_the_boolean);

    // Integers
    let result = 10;
    let age:u32 = 10;
    let sum:i32 = 5 - 15;
    let mark:isize = 10;
    let count:usize = 30;
    println!("The result is {}", result);
    println!("Sum is {} and age is {}", sum, age);
    println!("Mark is {} and count is {}", mark, count);
    
    /* if you not use this, then compiler will give you a warning:
    
    warning: unused variable: `age`
    --> 2_data_types.rs:20:9
    |
    20 |     let age:u8 = 255;
    |         ^^^ help: if this is intentional, prefix it with an underscore: `_age`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

    warning: 1 warning emitted
    


    let age:u8 = 255;
    */
    
    /*   The following lines will cause an error because the values exceed the maximum limit of u8 (255).
    let weight:u8 = 256;
    let height:u8= 257;
    let score:u8 = 258;
    println!("Age is {}", age);
    println!("Weight is {}", weight);
    println!("Height is {}", height);
    println!("Score is {}", score);
    */

    let result = 10.03; // f64 by default
    let interest:f32 = 8.35;
    let cost:f64 = 15021.245;

    println!("The result is {}", result);
    println!("The interest is {}", interest);
    println!("The cost is {}", cost);

    /*
    Automatic type casting like this is not allowed in Rust, you need to explicitly specify the type if you want to use a different one.

    let interest:f32 = 10;
    println!("The interest is {}", interest);
    */

    let float_with_separator = 11_000.555_001;
    println!("The float with separator is {}", float_with_separator);

    let int_with_separator = 50_000;
    println!("The integer with separator is {}", int_with_separator);

    let is_fun:bool = true;
    println!("Is learning Rust fun? {}", is_fun);

    let special_character = '@';
    let alphabet:char = 'A';

    println!("The special character is {}", special_character);
    println!("The alphabet is {}", alphabet);
}