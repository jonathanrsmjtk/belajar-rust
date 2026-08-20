fn main() {
    let num:i32 = 5;
    if num > 0 {
        println!("Number is positive");
    }

    let num = 12;
    if num % 2 == 0 {
        println!("Even");
    }
    else {
        println!("Odd");
    }

    let num = 2;
    if num > 0 {
        println!("Num is positive");
    }
    else if num < 0 {
        println!("Num is negative");
    }
    else {
        println!("Neither num is negative or positive");
    }

    let country_code = "ID";
    let country = match country_code {
        "ID" => {println!("Found name of ID!"); "Indonesia"},
        "MY" => "Malaysia",
        "US" => "United States of America",
        _ => "Unknown"
    };

    println!("Country name is {}", country);
}