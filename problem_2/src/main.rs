use std::env;
use std::fs;
use std::str::FromStr;

struct Level {
  reports: Vec<i32>,
  

}

fn read_vectors(data: String) -> Vec<Level> {
  let mut levels: Vec<Level> = Vec::new();

  for line in data.lines() {

    // Parse out two integers, removing everything else.
    let parsed: Vec<i32> = line
      .split_whitespace()
      .filter_map(|s| i32::from_str(s).ok())
      .collect();

    let parsed_level = Level {
      reports: parsed,
    };
    levels.push(parsed_level);
  }

  levels
}

fn check_report(reports: &Vec<i32>) -> (bool, usize) {
  let mut increasing: bool = false;
  let mut decreasing: bool = false;

  for i in 0..reports.len() {

    if i < reports.len() - 1 {
      // Initial Checks
      if !increasing && !decreasing {
        if reports[i] > reports[i + 1] {
          decreasing = true;
        } else if reports[i] < reports[i + 1] {
          increasing = true;
        }
      }

      // If no longer increasing/decreasing 
      let difference = reports[i] - reports[i + 1];

      let mut idx:usize = i;
      if increasing && reports[i] > reports[i + 1] {
          // Calculate Offending Index
          if i + 1 == reports.len() - 1 || (increasing && reports[i] < reports[i + 2]) {
            idx = i + 1;
          }

          return (false, idx);

      } else if decreasing && reports[i] < reports[i + 1] {
          // Calculate Offending Index
          if i + 1 == reports.len() - 1 || (decreasing && reports[i] > reports[i + 2]) {
            idx = i + 1;
          }

          return (false, idx);

      } else if difference.abs() < 1 || difference.abs() > 3 {
          if i + 1 != reports.len() - 1 {
            let alt_diff = reports[i] - reports[i + 2];
            if alt_diff.abs() >= 1 && alt_diff.abs() <= 3 {
              idx = i + 1;
            }
          } else if i + 1 == reports.len() - 1 {
            idx = i + 1;
          }

          return (false, idx);

      }

    }
  }

  return (true, 0)
}

fn check_safe(level: Level) -> bool {
  let mut safe: bool = false;
  let mut idx: usize = 0;

  // idx will be the offending index if failed
  (safe, idx) = check_report(&level.reports);
  
  // Dampen if the first check failed, remove the offender.
  if !safe {
    let mut dampened = Level {
        reports: level.reports,  // Move to a dampened vector
    };

    dampened.reports.remove(idx);

    (safe, idx) = check_report(&dampened.reports);

  }

  return safe;
}

fn main() {
    
  // Expect Argument[1] to be a file.
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];

  let file_data = fs::read_to_string(file_path)
    .expect("Could not open file.");

  let levels = read_vectors(file_data);

  let mut count:i32 = 0;
  for level in levels {
      let is_safe = check_safe(level);
      if is_safe {
          count = count + 1;

      }

  }

  println!("{count} levels were safe.");

}
