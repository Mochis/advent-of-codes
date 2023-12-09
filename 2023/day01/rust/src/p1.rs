use std::fs::File;
use std::io::{self, BufRead};

fn main() {
  let input_lines = lines("../input1.txt");
  let mut sum = 0;
  for line in input_lines {
    sum += calibration_value(line.unwrap().as_str())
  }
  println!("The solution is: {}", sum);
}

fn calibration_value(enc_calib_value: &str) -> u32 {
  let mut is_first_digit = true;
  let mut first: char = ' ';
  let mut last: char = ' ';
  for ch in enc_calib_value.chars() {
    if ch.is_digit(10) && is_first_digit {
      first = ch;
      is_first_digit = false;
    } else if ch.is_digit(10) {
      last = ch;
    }
  }
  last = if last == ' ' { first } else { last };
  return first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
}

fn lines(path: &str) -> io::Lines<io::BufReader<File>> {
  let file = File::open(path)
      .expect("This program expects the file being included in the project");
  io::BufReader::new(file).lines()
}