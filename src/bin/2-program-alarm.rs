// Example Usage:
// $ 1-rocket-equation ./data/1-input

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn execute (mem: &mut Vec<usize>) {
  let mut i = 0;

  loop {
    let op_code = mem[i];
    match op_code {
      1 => {
        let arg_a = mem[i + 1];
        let arg_b = mem[i + 2];
        let dest = mem[i + 3];
        mem[dest] = mem[arg_a] + mem[arg_b];
        i += 4;
      },
      2 => {
        let arg_a = mem[i + 1];
        let arg_b = mem[i + 2];
        let dest = mem[i + 3];
        mem[dest] = mem[arg_a] * mem[arg_b];
        i += 4;
      },
      99 => break,
      _ => panic!("Invalid op_code {}", op_code)
    }
  }
}

fn main () {
  let path = match env::args().nth(1) {
    None => panic!("File path must be passed"),
    Some(str) => str,
  };

  let file = File::open(path).unwrap();
  let mut reader = BufReader::new(file);
  let mut contents = String::new();
  reader.read_to_string(&mut contents).unwrap();

  let mut mem: Vec<usize> = contents
    .split(',')
    .filter_map(|x| x.parse::<usize>().ok())
    .collect();

  mem[1] = 12;
  mem[2] = 2;
  execute(&mut mem);
  println!("mem[0] = {}", mem[0]);
}
