// Example Usage:
// $ 1-rocket-equation ./data/1-input

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn execute (
  orig_mem: &Vec<usize>,
  param_a: usize,
  param_b: usize
) -> Result<usize, &'static str> {
  let size = orig_mem.len();
  let mut mem = orig_mem.to_vec();
  let mut i = 0;

  mem[1] = param_a;
  mem[2] = param_b;

  loop {
    if i >= size {
      return Err("Invalid access")
    }

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
      _ => return Err("Invalid operation")
    }
  }

  return Ok(mem[0]);
}

fn find_output (mem: &Vec<usize>, needle: usize) -> (usize, usize) {
  let len = mem.len();
  for x in 0..len {
    for y in 0..len {
      match execute(&mem, x, y) {
        Ok(p) if p == needle => return (x, y),
        Err(msg) => println!("Err: {}", msg),
        _ => ()
      }
    }
  }

  panic!("No params found for given input");
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

  let mem: Vec<usize> = contents
    .split(',')
    .filter_map(|x| x.parse::<usize>().ok())
    .collect();
  let (noun, verb) = find_output(&mem, 19690720);
  println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
}
