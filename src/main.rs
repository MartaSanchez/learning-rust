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
//---------------------------------------------
pub mod generic_min;
use crate::generic_min::min;
//---------------------------------------------
pub mod counter;
use crate::counter::Counter;
//---------------------------------------------
pub mod rot13;
use crate::rot13::rot_decoder_funtion;
//---------------------------------------------
pub mod log_filter;



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
    println!("---------------------------------------------");

    //Generic min exercise
    println!("Generic min exercise test");
    // Int
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
    // Chars
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
    // String
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
    println!("---------------------------------------------");
    //Counter exercise
    // Int
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }
    // String
    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
    println!("---------------------------------------------");
    //ROT13 exercise
    rot_decoder_funtion();
    println!("---------------------------------------------");
    //Log filter exercise
}

