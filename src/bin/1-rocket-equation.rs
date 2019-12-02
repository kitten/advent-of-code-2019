// Example Usage:
// $ 1-rocket-equation ./data/1-input

use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;

fn main () {
  let path = match env::args().nth(1) {
    None => panic!("File path must be passed"),
    Some(str) => str,
  };

  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);
  let fuel = reader
    .lines()
    .filter_map(|x| x.unwrap().parse::<i32>().ok())
    .map(|x| x / 3 - 2)
    .sum::<i32>();

  println!("Total fuel consumption: {}", fuel);
}
