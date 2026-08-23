fn main() {
    let tuple: (i32, f64, u8) = (-325, 4.9, 22);
    println!("{:?}", tuple);
    println!("integer is {:?}", tuple.0);
    println!("float is {:?}", tuple.1);
    println!("unsigned int is {:?}", tuple.2);

    let b: (i32, bool, f64) = (110, true, 10.9);
    print(b);
    print_destruct(b);
}

fn print(x: (i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}

fn print_destruct(x: (i32, bool, f64)) { // destruct (unpack values of tuple)
    println!("Inside print method");
    let (age, is_male, cgpa) = x;
    println!("Age is {}, is_male? {}, cgpa is {}", age, is_male, cgpa);
}