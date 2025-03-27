pub mod fibonacci_exercise;
use crate:: fibonacci_exercise::fib;

pub mod collatz_sequence_exercise;
use crate:: collatz_sequence_exercise::collatz_length;

pub mod geometry_exercise;
use crate::geometry_exercise::magnitude;
use crate::geometry_exercise::normalize;


fn main() {
    println!("Hello, world!");
    //Fibonacci
    let n = 20;
    println!("Fibonacci({n}) = {}", fib(n));
    // Length Collatz
    println!("Length Collatz: {}", collatz_length(11)); 
    //Geometry
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

}

