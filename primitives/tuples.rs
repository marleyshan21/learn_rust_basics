// Learning about tuples

// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html


// A tuple is a collection of values of different types

use std::fmt;

// Tuples can be used as function arguments and return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {

    // "let" can be used to assign the individual values of tuples to other variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}


struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix { // implement display type for matrix type

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "( {} {} ) \n( {} {} )", self.0, self.1, self.2, self.3)

    }

}


fn main(){


    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // we can extarct values from tuples using indexing
    println!("Long tuple first value = {}", long_tuple.0);
    println!("Long tuple fourth value = {}", long_tuple.3);

    // use functions this way
    let pair = (1, true);
    println!("The pair is {:?}", pair);

    // reverse it
    println!("The reversed pair is {:?}", reverse(pair));


    // exercise 1

    // print matrix
    println!("Print matrix");
    let matrix =  Matrix(1.1,  1.2, 2.1, 2.2);
    println!("{}", matrix);


}