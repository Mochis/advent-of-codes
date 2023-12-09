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

fn main() {
  let input_lines = lines("../../input2.txt");
  let mut sum = 0;
  for line in input_lines {
    sum += evaluate_game(line.unwrap().as_str()) 
  }
  println!("The solution is: {}", sum);
}

fn evaluate_game(game_line: &str) -> u32 {
  let mut game_parser = game_line.chars();

  // Skip to game set
  while game_parser.next() != Some(':') {}

  power(game_parser)
}

fn power(mut game_parser: Chars) -> u32 {
  let mut optimum_bag = Bag { red: 0, green: 0, blue: 0 };
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

    match cube_color_of(color.as_str()) {
      CubeColor::Red   => if qty > optimum_bag.red { optimum_bag.red = qty },
      CubeColor::Green => if qty > optimum_bag.green { optimum_bag.green = qty },
      CubeColor::Blue  => if qty > optimum_bag.blue { optimum_bag.blue = qty }
    };

    if ch.is_none()  {
      break optimum_bag.red * optimum_bag.green * optimum_bag.blue
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
