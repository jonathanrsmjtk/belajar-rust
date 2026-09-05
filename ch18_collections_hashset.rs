use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();
    names.insert("Budi");
    names.insert("Agus");
    names.insert("Endang");
    names.insert("Budi"); // akan diskip karena duplikat
    println!("{:?}", names);

    for name in names.iter() {
        println!("{}", name);
    }

    match names.get(&"Budi") {
        Some(value) => {
            println!("{} is available", value)
        },

        None => {
            println!("No name found");
        }
    }

    if names.contains(&"Agus") {
        println!("Agus is available");
    }
    else {
        println!("No name found");
    }

    println!("Size of names {}", names.len());
    names.remove(&"Agus");
    println!("Size of names after remove {}, bye Agus", names.len());
}