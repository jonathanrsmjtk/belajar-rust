fn main() {
    let fees = 25_000;
    let salary:f64 = 35_000.00;
    println!("The fees are {} and the salary is {}", fees, salary);

    /*
    Error for this:
    error[E0384]: cannot assign twice to immutable variable `fees`
    --> 3_variables.rs:6:5
    |
    2 |     let fees = 25_000;
    |         ---- first assignment to `fees`
    ...
    6 |     fees = 35_000;
    |     ^^^^^^^^^^^^^ cannot assign twice to immutable variable
    |
    help: consider making this binding mutable
    |
    2 |     let mut fees = 25_000;
    |         +++

    error: aborting due to 1 previous error

    For more information about this error, try `rustc --explain E0384`.

    fees = 35_000;
    println!("Fees changed to {}", fees);
    
    */

    let mut fees:i32 = 25_000;
    println!("The fees are {}", fees);
    fees = 35_000;
    println!("Fees changed to {}", fees);
}