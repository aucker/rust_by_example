use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
fn main() {

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("number of the elements in the array: {}", xs.len());
    
    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices 
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice 
    // ending_index is one more than the last position in the slice 
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);


    println!("Hello, world!");
}
