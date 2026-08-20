fn main () {
    let mut a = 10;
    let mut b = 20;
    println!("Assume a = {} and b {}", a, b);
    println!("\nRelational operators");
    println!("a > b is {}", a > b);
    println!("a < b is {}", a < b);
    println!("a >= b is {}", a >= b);
    println!("a <= b is {}", a <= b);
    println!("a == b is {}", a == b);
    println!("a != b is {}", a != b);
    println!("\nLogical Operators");
    println!("a > 10 && b > 10 is {}", a > 10 && b > 10);
    println!("a > 10 || b > 10 is {}", a > 10 || b > 10);
    println!("!a > 10 is {}", !(a > 10));

    a = 2;
    b = 3;
    println!("\nBitwise Operators");
    println!("a & b is {}", a & b);
    println!("a | b is {}", a | b);
    println!("a ^ b is {}", a ^ b);
    println!("!b is {}", !b);
    println!("a << 1 is {}", a << 1);
    println!("a >> 1 is {}", a >> 1);
}