// An iterator helps to iterate over a collection of values such as arrays, vectors, maps, etc.

fn main() {
    let a = [10, 20, 30];

    let mut iter = a.iter();
    println!("{:?}", iter);

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let iter = a.iter();
    for data in iter {
        print!("{}\t", data)
    }

/*
    iter(): gives an iterator over &T (reference to T),  uses the concept of borrowing
    into_iter(): gives an iterator over T, uses the concept of ownership
    iter_mut(): gives an iterator over &mut T
*/
    let names = vec!["Andi", "Budi", "Faisal"];
    for name in names.iter() {
        match name {
            &"Faisal" => println!("There is a crustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("{:?}", names);

    let names = vec!["Andi", "Budi", "Faisal"];
    for name in names.into_iter() {
        match name {
            "Faisal" => println!("There is a crustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("{:?}", names); // -> ini pasti error, karena collection names sudah kena ownership move

    let mut names = vec!["Andi", "Budi", "Faisal"];
    for name in names.iter_mut() {
        match name {
            &mut "Faisal" => println!("There is a crustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("{:?}", names);
}