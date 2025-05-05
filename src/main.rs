pub mod fibonacci;
use crate:: fibonacci::fib;
//---------------------------------------------
pub mod collatz_sequence;
use crate:: collatz_sequence::collatz_length;
//---------------------------------------------
pub mod nested_arrays;
use crate::nested_arrays::transpose;
//---------------------------------------------
pub mod geometry;
use crate::geometry::{magnitude, normalize};
//---------------------------------------------
pub mod elevator_events;
use crate::elevator_events::{
    car_arrived,
    car_door_opened,
    car_door_closed,
    lobby_call_button_pressed,
    car_floor_button_pressed,
    Direction
};
//---------------------------------------------
pub mod expression_evaluation;
use crate::expression_evaluation::hello_from_file;

//---------------------------------------------
pub mod logger_trait;
use crate::logger_trait::{VerbosityFilter, Logger, StderrLogger};
fn main() {
    println!("Hello, world!");
    println!("---------------------------------------------");
    //Fibonacci
    let n = 20;
    println!("Fibonacci({n}) = {}", fib(n));
    println!("---------------------------------------------");

    // Length Collatz
    println!("Length Collatz: {}", collatz_length(11)); 
    println!("---------------------------------------------");

    // Nested arrays
    let matrix = [
        [101, 102, 103], 
        [201, 202, 203],
        [301, 302, 303],
    ];
    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
    println!("---------------------------------------------");

    //Geometry
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
    println!("---------------------------------------------");

    //Elevator
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
    println!("---------------------------------------------");

    //Expression evaluation
    hello_from_file();
    println!("---------------------------------------------");

    //Logger trait exercise
    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");

}

