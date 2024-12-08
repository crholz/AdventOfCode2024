use std::env;
use std::fs;
use regex::Regex;

fn main() {
    // Expect Argument[1] to be a file.
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];

  let file_data = fs::read_to_string(file_path)
    .expect("Could not open file.");

  // Process File Data: Into Splits of Do/Don't
  let splits = file_data.split("do");
    
  // Create the regex pattern for the mul(\d, \d)
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

  let mut total: i32 = 0;
  for split in splits {
    let base = "don't";
    let filter = ["do", &split[0..3]].join("");
    if filter != base {
      for (_, [mul1, mul2]) in re.captures_iter(&split).map(|c| c.extract()) {
        total += mul1.parse::<i32>().unwrap() * mul2.parse::<i32>().unwrap();
      }
    }
  }

  println!("Total parsed from file: {total}.")
}
