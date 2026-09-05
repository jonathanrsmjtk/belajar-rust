use std::collections::HashMap;

fn main() {
    let mut state_codes = HashMap::new();
    state_codes.insert("IDN", "Indonesia");
    state_codes.insert("IND", "India");
    println!("{:?}", state_codes);
    println!("Size of state_codes is {}", state_codes.len());
    
    match state_codes.get(&"IDN") {
        Some(value) => {
            println!("Value of IDN is {}", value);
        }
        None => {
            println!("Nothing found");
        }
    }

    for (key, val) in state_codes.iter() {
        println!("key: {}, val: {}", key, val);
    }

    if state_codes.contains_key(&"USA") {
        println!("Key found");
    }
    else {
        println!("Key not found");
    }

    state_codes.insert("SGP", "Singapore");
    println!("Size of state_codes is {}", state_codes.len());
    state_codes.remove(&"SGP");
    println!("Size of state_codes after remove is {}", state_codes.len());
}