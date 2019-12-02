// Example Usage:
// $ 1-2-rocket-equation ./data/1-input

use std::io::{BufReader,BufRead};
use std::fs::File;
use std::env;

fn calculate_fuel (x: i32) -> i32 {
  let res = x / 3 - 2;
  if res > 0 {
    res + calculate_fuel(res)
  } else {
    0
  }
}

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
    .map(calculate_fuel)
    .fold(0, |acc, x| acc + x);

  println!("Total fuel consumption: {}", fuel);
}
