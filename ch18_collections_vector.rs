// Collection secara umum - Vector, HashMap, dan HashSet

/*
Vector:
- A Vector can grow or shrink at runtime.
- A Vector is a homogeneous collection.
- A Vector stores data as sequence of elements in a particular order. Every element in a Vector is assigned a unique index number. The index starts from 0 and goes up to n-1 where, n is the size of the collection. For example, in a collection of 5 elements, the first element will be at index 0 and the last element will be at index 4.
- A Vector will only append values to (or near) the end. In other words, a Vector can be used to implement a stack.
- Memory for a Vector is allocated in the heap.
*/

fn main() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);

    println!("Size of vector is {}", v.len());
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    /*
    Ini pasti error
    let v = vec![1,2,3,"hello"];
    Karena vector cuma bisa berisi 1 macam tipe data
     */
    println!("{:?}", v);

    let mut v = vec![10, 20, 30];
    println!("Size of v is {}", v.len());
    v.remove(1);
    println!("{:?}", v);
    if v.contains(&10) {
        println!("Found 10");
    }

    println!("{}", v[1]);

    v.push(40);
    v.push(50);
    v.push(100);
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v);
}