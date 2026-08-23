/* 
Borrowing

Adalah sebuah konsep transfer kontrol atas variabel atau value ke fugnsi lain secara sementara.
Hal ini dilakukan dengan passing reference ketimbang melakukan passing variable atau value.
Setelah fungsi selesai eksekusi, variabel atau value akan dikembalikan ke owner nya.
*/

fn main() {
    let v = vec![10, 20, 30];
    print_vector(&v); // borrow
    println!("Printing the value from main() v[0] = {}", v[0]); // sudah dikembalikan ke v

    // mutable
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i);

    let mut name:String = String::from("Hi");
    display(&mut name);
    println!("The value of name after modification is : {}", name);

    let mut x = vec![10, 20, 30];
    change_vec_value(&mut x, 3);
    println!("{:?}", x);
}

fn print_vector(x: &Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}

fn add_one(e: &mut i32) {
    *e += 1
}

fn display(param_name: &mut String) {
    println!("Param name value is: {}", param_name);
    param_name.push_str(" Bambang");
}

fn change_vec_value(vec: &mut Vec<i32>, value: i32) {
    vec[0] = value;
}