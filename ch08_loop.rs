fn main() {
    for x in 1..11 { // 11 is not included
        if x == 5 {
            continue;
        }
        println!("{}", x);
    }

    let mut x = 0;
    while x < 10 {
        x += 1;
        println!("Inside loop, x is {}", x);
    }
    println!("Outside loop, x is {}", x);

    x = 0;
    loop {
        x += 1;
        println!("x = {}", x);
        if x == 20 {
            break;
        }
    }

    let mut count = 0;
    for num in 1..20 {
        if num % 2 == 0 {
            continue;
        }
        count += 1;
    }
    println!("Count of odd numbers is {}", count);
}