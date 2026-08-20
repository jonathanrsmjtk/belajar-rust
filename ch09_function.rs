fn fn_hello() {
    println!("Hello from fn hello()");
}

fn main() {
    fn_hello();
    fn_get_pi();
    mutate_number_to_zero(30);
    

    let mut number = 5;
    multiply_with_two(&mut number); // kalau multiply_with_two(&mut number); maka hasilnya jadi 5
    println!("The value of number is {}", number);

    let number = 3;
    multiply_with_three(number); // kalau multiply_with_three(number); maka hasilnya jadi 3
    println!("The value of number is {}", number);

    let number = multiply_with_three(3);
    println!("The value of number is {}", number);

    let name:String = String::from("Ayam");
    display(name);
}

fn fn_get_pi() -> f64 {
    22.0 / 7.0
}

fn mutate_number_to_zero(mut param_number: i32) { // param mesti mut atau gak bakal error
    param_number = param_number * 0;
    println!("param_number value is {}", param_number)
}

// pass by reference
/*
param_number adalah reference ke number, bukan value dari number. Jadi kalau param_number diubah, maka number juga akan berubah.
*/
fn multiply_with_two(param_number:&mut i32) {
    *param_number *= 2; // dereferencing
}

fn multiply_with_three(mut param_number: i32) -> i32{
    param_number *= 3;
    return param_number;
}

fn display(param_name: String) {
    println!("param_name value is {}", param_name)
}