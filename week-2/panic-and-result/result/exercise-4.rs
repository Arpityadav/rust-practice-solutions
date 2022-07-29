use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().map(|num| num +2)
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}


// Second half of the exercise
use std::num::ParseIntError;

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().and_then(|num| Ok(num +2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}