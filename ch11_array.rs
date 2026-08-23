fn main() {
    let arr: [i32; 4] = [10, 20, 30, 40];
    println!("Array is {:?}", arr);
    println!("Array size is {}", arr.len());

    let arr = [10, 20, 30, 40];
    println!("Array is {:?}", arr);
    println!("Array size is {}", arr.len());

    let arr2: [i32; 4] = [-1; 4];
    println!("Array is {:?}", arr2);
    println!("Array size is {}", arr2.len());

    for index in 0..4 {
        println!("index is: {}, value is : {}", index, arr[index])
    }

    for val in arr.iter() {
        println!("value is : {}", val)
    }

    let mut arr: [i32;4] = [10, 20, 30, 40];
    arr[1] = 0;
    println!("{:?}", arr);

    let arr = [10, 20, 30];
    update(arr);

    println!("Inside main {:?}", arr);

    let mut arr = [10, 20, 30];
    update_with_ref(&mut arr);

    println!("Inside main {:?}", arr);

    /*
    kayak gini bakal error 
    let N: usize = 20;
    */
    const N: usize = 20; // jadi, gunakan saja ini
    let arr = [0; N];
    println!("{}", arr[10])
}

fn update(mut arr: [i32; 3]) {
    for i in 0..3 {
        arr[i] = 0
    }
    println!("Inside update {:?}", arr)
}

fn update_with_ref(arr:&mut [i32; 3]) {
    for i in 0..3 {
        arr[i] = 0
    }
    println!("Inside update_with_ref {:?}", arr);
}