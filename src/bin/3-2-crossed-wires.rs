// Example Usage:
// $ 1-rocket-equation ./data/1-input

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

enum WireMove {
  Right(u32),
  Up(u32),
  Left(u32),
  Down(u32),
}

fn parse_write_str (wire: &String) -> Vec<WireMove> {
  return wire
    .trim()
    .split(',')
    .filter(|s| s.len() > 1)
    .filter_map(|s| {
      let (direction, moves) = s.split_at(1);
      let moves_num = moves.parse::<u32>().unwrap();
      match direction {
        "R" => Some(WireMove::Right(moves_num)),
        "U" => Some(WireMove::Up(moves_num)),
        "L" => Some(WireMove::Left(moves_num)),
        "D" => Some(WireMove::Down(moves_num)),
        _ => None
      }
    })
    .collect();
}

fn move_to_magnitude (m: WireMove) -> (i32, i32, u32) {
  return match m {
    WireMove::Right(d) => (1, 0, d),
    WireMove::Up(d) => (0, 1, d),
    WireMove::Left(d) => (-1, 0, d),
    WireMove::Down(d) => (0, -1, d),
  }
}

fn main () {
  let path = match env::args().nth(1) {
    None => panic!("File path must be passed"),
    Some(str) => str,
  };

  let file = File::open(path).unwrap();
  let mut reader = BufReader::new(file);
  let mut str_a = String::new();
  let mut str_b = String::new();
  reader.read_line(&mut str_a).unwrap();
  reader.read_line(&mut str_b).unwrap();

  let wire_a = parse_write_str(&str_a);
  let wire_b = parse_write_str(&str_b);

  let mut grid = HashMap::new();
  let mut shortest_run = std::u32::MAX;

  let mut d: u32 = 0;
  let mut x = 0;
  let mut y = 0;

  for wire_move in wire_a {
    let (mag_x, mag_y, mag_l) = match wire_move {
      WireMove::Right(d) => (1, 0, d),
      WireMove::Up(d) => (0, 1, d),
      WireMove::Left(d) => (-1, 0, d),
      WireMove::Down(d) => (0, -1, d),
    };

    for _i in 1..(mag_l + 1) {
      d += 1;
      x += mag_x;
      y += mag_y;
      grid.insert((x, y), d);
    }
  }

  d = 0;
  x = 0;
  y = 0;

  for wire_move in wire_b {
    let (mag_x, mag_y, mag_l) = move_to_magnitude(wire_move);
    for _i in 1..(mag_l + 1) {
      d += 1;
      x += mag_x;
      y += mag_y;
      match grid.get(&(x, y)) {
        Some(d2) if d2 + d < shortest_run => shortest_run = d2 + d,
        _ => ()
      }
    }
  }

  println!("Shortest run to intersection = {}", shortest_run);
}
