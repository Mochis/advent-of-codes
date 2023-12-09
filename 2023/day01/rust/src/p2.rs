use std::fs::File;
use std::io::{self, BufRead};

const VALID_NUMBERS: [&str; 18] = [
  "1", "2", "3", "4", "5", "6", "7", "8", "9", 
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine"
];

fn main() {
  let input_lines = lines("../../input2.txt");
  let mut sum = 0;
  for line in input_lines {
    sum += calibration_value(line.unwrap().as_str())
  }
  println!("The solution is: {}", sum);
}

fn calibration_value(enc_calib_value: &str) -> u32 {
  let mut is_first_number = true;
  let mut first = "";
  let mut last = "";

  for (i, _) in enc_calib_value.chars().enumerate() {
    match find_number(&enc_calib_value[i..], VALID_NUMBERS.to_vec(), String::new()) {
      Some(number) if is_first_number => {
        first = number;
        is_first_number = false;
      },
      Some(number) => last = number,
      None => continue
    }
  }
  last = if last.is_empty() { first } else { last };

  return translate_to_int(first).unwrap() * 10 + translate_to_int(last).unwrap();
}

fn find_number<'a>(enc_calib_value: &str, options: Vec<&'a str>, mut prefix: String) -> Option<&'a str> {
  let mut prob_options: Vec<&str> = options;
  for (i, ch) in enc_calib_value.chars().enumerate() {
    prefix.push(ch);
    prob_options = filter_options_by_prefix(&prefix, prob_options);
    return match prob_options {
      opts if opts.len() == 0 => None,
      opts if opts.len() == 1 && opts.get(0).unwrap() == &prefix => Some(opts.get(0).unwrap()),
      _ => find_number(&enc_calib_value[i+1..], prob_options, prefix)
    }
  }
  None
}

fn filter_options_by_prefix<'a>(str_number: &str, options: Vec<&'a str>) -> Vec<&'a str> {
  let mut filtered_opts = Vec::new();
  for opt in options {
    if opt.starts_with(str_number) {
      filtered_opts.push(opt);
    }
  }
  filtered_opts
}

fn translate_to_int(str_number: &str) -> Option<u32> {
  match str_number {
    "one"   | "1" => Some(1),
    "two"   | "2" => Some(2),
    "three" | "3" => Some(3),
    "four"  | "4" => Some(4),
    "five"  | "5" => Some(5),
    "six"   | "6" => Some(6),
    "seven" | "7" => Some(7),
    "eight" | "8" => Some(8),
    "nine"  | "9" => Some(9),
    _ => None
  }
}

fn lines(path: &str) -> io::Lines<io::BufReader<File>> {
  let file = File::open(path)
      .expect("This program expects the file being included in the project");
  io::BufReader::new(file).lines()
}