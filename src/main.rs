pub mod fibonacci;
use crate:: fibonacci::fib;

pub mod collatz_sequence;
use crate:: collatz_sequence::collatz_length;

pub mod nested_arrays;
use crate::nested_arrays::transpose;

pub mod geometry;
use crate::geometry::magnitude;
use crate::geometry::normalize;


fn main() {
    println!("Hello, world!");
    //Fibonacci
    let n = 20;
    println!("Fibonacci({n}) = {}", fib(n));
    // Length Collatz
    println!("Length Collatz: {}", collatz_length(11)); 
    // Nested arrays
    let matrix = [
        [101, 102, 103], 
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);

    //Geometry
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

}

