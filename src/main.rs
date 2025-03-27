pub mod fibonacci_exercise;
use crate:: fibonacci_exercise::fib;

fn main() {
    println!("Hello, world!");
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}

