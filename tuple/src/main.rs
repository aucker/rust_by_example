use std::fmt;
fn reverse(pair: (i32, bool)) -> (bool, i32) {

    let (integer, boolean) = pair;
    (boolean, integer)
}


#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Add a transpose function to swap the matrix
fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = matrix;
    Matrix(a, c, b, d)
}

// Recap: Add the `fmt::Display` trait to the Matrix struct
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
    }
}
fn main() {

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("long_tuple first value: {}", long_tuple.0);
    println!("long_tuple second valur: {}", long_tuple.1);


    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    // tuple are printble
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // but long Tuples cannot be printed
    let too_long_tuples = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuples);

    let pair = (1, true);
    println!("pair is {:?}",pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, b, d);
    
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
    println!("Hello, world!");
}
