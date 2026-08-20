fn main() {
    const USER_LIMIT:i32 = 100;
    const PI:f32 = 3.14;

    println!("The user limit is {}", USER_LIMIT);
    println!("The value of PI is {}", PI);

    // Shadowing variables
    let salary = 100.00;
    let salary = 1.40;
    println!("The salary is {}", salary);

    let uname = "Jonathan";
    let uname = uname.len();
    println!("Name changed to integer: {}", uname);

    // In constant, shadowing is not possible
    /*
    This will produce error:
    error[E0428]: the name `NAME` is defined multiple times
    --> 4_constant.rs:20:5
    |
    19 |     const NAME:&str = "Jonathan";
    |     ----------------------------- previous definition of the value `NAME` here
    20 |     const NAME:usize = NAME.len();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `NAME` redefined here
    |
    = note: `NAME` must be defined only once in the value namespace of this block

    const NAME:&str = "Jonathan";
    const NAME:usize = NAME.len();
    println!("Name changed to integer: {}", NAME);
    */
}