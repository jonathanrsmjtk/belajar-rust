fn main() {
    let company:&str = "PerusahaanBapakKau";
    let location:&str = "Di hatimu ku berada";
    println!("Company: {}, location: {}", company, location);

    // define that these strings are static
    let company:&'static str = "PT. Mencari Cinta Sejati";
    let location:&'static str = "Penjara Cinta";
    println!("Company: {}, location: {}", company, location);

    /*
    The String object type is provided in Standard Library. Unlike string literal, 
    the string object type is not a part of the core language. It is defined as 
    public structure in standard library pub struct String. String is a growable collection. 
    It is mutable and UTF-8 encoded type. The String object type can be used to represent 
    string values that are provided at runtime. String object is allocated in the heap.
    */
    let empty_string = String::new();
    println!("Length of `empty_string` is {}", empty_string.len());

    let content_string = String::from(company);
    println!("Length of `content_string` is {}", content_string.len());
    
    let mut z = String::new();
    z.push_str("Hai");
    println!("{}", z);

    let name1 = "Halo Indon,
    Halo";
    println!("{}", name1);

    let num1 = 12345.to_string();
    println!("{}", num1);

    let name1 = "Halo Indon,
    Halo".replace("Halo", "Hai");
    println!("{}", name1);

    let example_string = String::from("example string");
    print_literal(example_string.as_str()); // atau print_literal(&example_string); konsep borrow

    let mut company = "Tutorial".to_string();
    company.push('s');
    println!("{}", company);

    company = "Tutorial".to_string();
    company.push_str(" mencari janda bohay");
    println!("{}", company);

    let full_name = "Tutorial biawak \r\n";
    println!("Before trim ");
    println!("length is {}", full_name.len());
    println!();
    println!("After trim ");
    println!("{}", full_name.trim());
    println!("length is {}", full_name.trim().len());

    let message = "Bapakmu sedang cari LC di sana".to_string();
    let mut i = 1;

    for token in message.split_whitespace() {
        println!("{} {}", i, token);
        i += 1;
    }

    let names = "Bambang, Budi, Indra";
    for token in names.split(", ") {
        println!("token is {}", token);
    }

    println!("\n");
    let tokens:Vec<&str> = names.split(", ").collect();
    println!("First is {}", tokens[0]);
    println!("Second is {}", tokens[1]);
    println!("Third is {}", tokens[2]);

    let n1 = "Tutorials";
    for n in n1.chars(){
        println!("{}", n);
    }

    let n1 = "Lele".to_string();
    let n2 = "Goreng".to_string();
    let n3 = n1 + &n2; // n1 + n2 bakal error, konsep borrowing
    println!("{}", n3);

    /* 
        Mesti define lagi n1 dan n2 kalau mau dipakai n3_format, kalau tidak 
        akan error kayak gini:
        error[E0382]: borrow of moved value: `n1`
            --> 5_string.rs:87:38
            |
            82 |     let n1 = "Lele".to_string();
            |         -- move occurs because `n1` has type `String`, which does not implement the `Copy` trait
            83 |     let n2 = "Goreng".to_string();
            84 |     let n3 = n1 + &n2; // n1 + n2 bakal error, konsep borrowing
            |              -- value moved here
            ...
            87 |     let n3_format = format!("{} {}", n1, n2);
            |                                      ^^ value borrowed here after move
            |
            help: consider cloning the value if the performance cost is acceptable
            |
            84 |     let n3 = n1.clone() + &n2; // n1 + n2 bakal error, konsep borrowing
            |                ++++++++

            error: aborting due to 1 previous error
    */
    let n1 = "Lele".to_string();
    let n2 = "Goreng".to_string();
    let n3_format = format!("{} {}", n1, n2);
    println!("{}", n3_format);
}

fn print_literal(data: &str) {
    println!("displaying string literal {}", data);
}