use std::fmt::Display;

struct Data<T> {
    value: T
}

struct Book {
    name:&'static str,
    id:u32
}

trait Printable {
    fn print(&self);
}

impl Printable for Book {
    fn print(&self) {
        println!("Printing book with id: {} and name {}", self.id, self.name);
    }
}

// Generic functions
// This able for the func to accept parameter with any data types.
fn print_pro<T:Display>(t:T) {
    println!("Inside print_pro generic function:");
    println!("{}", t);
}

fn main() {
    let mut vector_integer: Vec<i32> = vec![20, 30];
    vector_integer.push(40);
    // vector_integer.push("hi"); ini sudah pasti error, type sudah didefine i32 (generic collection)
    println!("{:?}", vector_integer);

    let t:Data<i32> = Data{value: 350};
    println!("value is {}", t.value);
    let t2:Data<String> = Data{value: "Budi".to_string()};
    println!("value is {}", t2.value);

    let b1 = Book {
        id: 1001,
        name: "Love story of us"
    };
    
    b1.print();

    print_pro(10 as u8);
    print_pro(20 as u16);
    print_pro("Hi people");
}