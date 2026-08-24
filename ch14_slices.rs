// Slice: Pointer ke blok memori

fn main() {
    let n1 = "IndonesiaNegaraku".to_string();
    println!("Length of string: {}", n1.len());
    let c1 = &n1[4..9];

    println!("{}", c1);

    let data = [10, 20, 30, 40, 50];
    use_slice(&data[1..4]);

    let mut data = [10, 20, 30, 40, 50];
    use_slice_mut(&mut data[1..4]);
    println!("{:?}", data);
}

fn use_slice(slice: &[i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
}

fn use_slice_mut(slice: &mut [i32]) {
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
    slice[0] = 1010
}