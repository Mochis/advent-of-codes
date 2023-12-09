use std::fs::File;
use std::io::{self, BufRead};
use std::str::Chars;

struct Bag {
  red: u32,
  green: u32,
  blue: u32
}

enum CubeColor {
  Red,
  Green,
  Blue
}

const THE_BAG: Bag = Bag { red: 12, green: 13, blue: 14 };

fn main() {
  let input_lines = lines("../../input1.txt");
  let mut sum = 0;
  for line in input_lines {
    match evaluate_game(line.unwrap().as_str()) {
      Some(x) => sum += x,
      None => continue
    }
  }
  println!("The solution is: {}", sum);
}

fn evaluate_game(game_line: &str) -> Option<u32> {
  let mut game_parser = game_line.chars();

  // Skip to gameId
  while game_parser.next() != Some(' ') {}

  let mut game_id_str = String::new();
  let mut ch_game_id = game_parser.next();
  while ch_game_id != Some(':') {
    game_id_str.push(ch_game_id.unwrap());
    ch_game_id = game_parser.next();
  }
  let game_id = game_id_str.parse::<u32>().unwrap();

  return if is_valid_game(game_parser) { Some(game_id) } else { None };
}

fn is_valid_game(mut game_parser: Chars) -> bool {
  loop {
    game_parser.next(); // Skip space
    let mut ch = game_parser.next();

    // Parse qty
    let mut qty_str = String::new();
    while ch.unwrap().is_digit(10) {
      qty_str.push(ch.unwrap());
      ch = game_parser.next();
    }
    let qty = qty_str.parse::<u32>().unwrap();
    
    // Parse color
    let mut color = String::new();
    ch = game_parser.next();
    while ch != Some(',') && ch != Some(';') && !ch.is_none() {
      color.push(ch.unwrap());
      ch = game_parser.next();
    }

    let pass = match cube_color_of(color.as_str()) {
      CubeColor::Red => qty <= THE_BAG.red,
      CubeColor::Green => qty <= THE_BAG.green,
      CubeColor::Blue => qty <= THE_BAG.blue
    };

    if !pass {
      break false
    } else if ch.is_none()  {
      break true
    }
  }
}

fn cube_color_of(str_color: &str) -> CubeColor {
  match str_color {
    "red" => CubeColor::Red,
    "green" => CubeColor::Green,
    "blue" => CubeColor::Blue,
    _ => panic!("That color does not exist")
  }
}

fn lines(path: &str) -> io::Lines<io::BufReader<File>> {
  let file = File::open(path)
      .expect("This program expects the file being included in the project");
  io::BufReader::new(file).lines()
}
