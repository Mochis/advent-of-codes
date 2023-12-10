use std::fs::File;
use std::io::{self, BufRead, Error};

// Algorithm Big O = O(n)

fn main() {
  let mut input_lines = lines("../../input1.txt");
  let mut sum = 0;
  let mut prev: Option<Result<String, Error>> = None;
  let mut curr = input_lines.next();
  let mut next = input_lines.next();
  loop {
    if next.is_none() {
      sum += partial_engine_parts(&prev, &curr, &next);
      break;
    } else {
      sum += partial_engine_parts(&prev, &curr, &next);
      prev = curr;
      curr = next;
      next = input_lines.next();
    }
  }
  
  println!("The solution is: {}", sum);
}

fn partial_engine_parts(prev: &Option<Result<String, Error>>, curr: &Option<Result<String, Error>>, next: &Option<Result<String, Error>>) -> i32 {
  let mut part_number = String::new();
  let mut parts_sum = 0;
  let mut take_into_account = false;

  let curr_string = unwrap(curr);

  for (i, ch) in curr_string.chars().enumerate() {
    if ch.is_digit(10) {
      part_number.push(ch);
      if is_adjacent_to_symbol(i, prev, curr, next) {
        take_into_account = true;
      }
    } else {
      if take_into_account {
        parts_sum += part_number.parse::<i32>().unwrap();
        take_into_account = false;
      }
      part_number.clear();
    }
  }
  if take_into_account { parts_sum += part_number.parse::<i32>().unwrap(); }
  
  parts_sum
}

fn is_adjacent_to_symbol(idx: usize, up: &Option<Result<String, Error>>, curr: &Option<Result<String, Error>>, down: &Option<Result<String, Error>>) -> bool {
  let curr_str = unwrap(curr);

  if up.is_none() {
    // look right, left, down and diagonals down
    let down_str = unwrap(down); 
    
    is_symbol(&curr_str[if idx == 0 { 0..1 } else { idx-1..idx }]) ||
    is_symbol(&down_str[if idx == 0 { 0..1 } else { idx-1..idx }]) ||
    is_symbol(&down_str[idx..idx+1]) ||
    (idx+1 < curr_str.len() && (
      is_symbol(&curr_str[idx+1..idx+2]) || 
      is_symbol(&down_str[idx+1..idx+2]))
    )
  } else if down.is_none() {
    // look right, left, up and diagonals up  
    let up_str = unwrap(up);

    is_symbol(&curr_str[if idx == 0 { 0..1 } else { idx-1..idx }]) ||
    is_symbol(&up_str[if idx == 0 { 0..1 } else { idx-1..idx }]) ||
    is_symbol(&up_str[idx..idx+1]) ||
    (idx+1 < curr_str.len() && (
      is_symbol(&curr_str[idx+1..idx+2]) || 
      is_symbol(&up_str[idx+1..idx+2]))
    )
  } else {
    // look all directions
    let up_str = unwrap(up);
    let down_str = unwrap(down);
    
    is_symbol(&curr_str[if idx == 0 { 0..1 } else { idx-1..idx }]) || // left
    is_symbol(&up_str[if idx == 0 { 0..1 } else { idx-1..idx }]) ||   // up left
    is_symbol(&up_str[idx..idx+1]) ||                                 // up
    is_symbol(&down_str[if idx == 0 { 0..1 } else { idx-1..idx }]) || // down left
    is_symbol(&down_str[idx..idx+1]) ||                               // down
    (idx+1 < curr_str.len() && (
      is_symbol(&curr_str[idx+1..idx+2]) || // right
      is_symbol(&up_str[idx+1..idx+2]) ||   // up right
      is_symbol(&down_str[idx+1..idx+2]))   // down right
    )   
  }
}

fn is_symbol(char_str: &str) -> bool {
  let ch = char_str.chars().next().unwrap();
  !ch.is_digit(10) && ch != '.'
}

fn unwrap(to_unwrap: &Option<Result<String, Error>>) -> &String {
  let unwrapped = to_unwrap.as_ref().unwrap().as_ref().unwrap();
  unwrapped
}

fn lines(path: &str) -> io::Lines<io::BufReader<File>> {
  let file = File::open(path)
      .expect("This program expects the file being included in the project");
  io::BufReader::new(file).lines()
}
