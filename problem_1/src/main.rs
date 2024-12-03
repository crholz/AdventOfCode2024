use std::env;
use std::fs;
use std::str::FromStr;

// calculate_differences
// Calcuate the difference score of two vectors to be sorted.
// @input in_1: The first vector to compare.
// @input in_2: The second input to compare.
fn calculate_differences(in_1: Vec<i32>, in_2: Vec<i32>) -> i32 {
  let mut distance: i32 = 0;

  // These are guranteed to be the same length.
  let mut sorted_vec_1: Vec<i32> = in_1;
  let mut sorted_vec_2: Vec<i32> = in_2;

  sorted_vec_1.sort();
  sorted_vec_2.sort();

  if sorted_vec_1.len() == sorted_vec_2.len() {
    let length = sorted_vec_1.len();

    for i in 0..length {
      let calculation = sorted_vec_1[i] - sorted_vec_2[i];
      distance += calculation.abs();
    }
  } else {
    panic!("Vectors are expected to be the same length but are not.")
  }

  // Return final distance
  distance

}

// count_in
// Count the number of occurances in a vector.
// @input target: The number to search for in vector arr.
// @input arr: A vector of numbers
// @returns: Number of occurances of target in arr.
fn count_in(target: i32, arr: Vec<i32>) -> i32 {
  let mut count: i32 = 0;
  for num in arr.iter() {
    if *num == target {
      count = count + 1;
    }
  }

  count

}

// calculate_sim
// Calculate the similarity scores of in_1 and in_2.
// @input in_1: The base vector that targets are found in.
// @input in_2: The vector that the number occurances will be found in.
// @returns integer which is the similarity score. Calculated as in_1[idx] * (Number of Occurances in in_2)
fn calculate_sim(in_1: Vec<i32>, in_2: Vec<i32>) -> i32 {
  // These vectors don't have to be sorted.
  let mut similarity: i32 = 0;

  // Potential optimization, cache similarity items.
  // Higher memory impact though.
  for num in in_1.iter() {
    similarity += num * count_in(*num, in_2.clone());
  }

  // Return the Similarity Score
  similarity
}

// read_vectors
// Read in data and return two vectors.
// @input data: String to parse from line-by-line
// @returns 2 Vectors.
fn read_vectors(data: String) -> (Vec<i32>, Vec<i32>) {
  let mut vec_1: Vec<i32> = Vec::new();
  let mut vec_2: Vec<i32> = Vec::new();

  for line in data.lines() {

    // Parse out two integers, removing everything else.
    let parsed: Vec<i32> = line
      .split_whitespace()
      .filter_map(|s| i32::from_str(s).ok())
      .collect();

    if parsed.len() == 2 {
      vec_1.push(parsed[0]);
      vec_2.push(parsed[1]);
    } else if parsed.len() == 1 {
      vec_1.push(parsed[0]);
      vec_2.push(0);

    } else {
      panic!("Could not parse the correct value of numbers from line.");
    }
  }

  // Return Vectors 1 and 2
  (vec_1, vec_2)
}

fn main() {
    
  // Expect Argument[1] to be a file.
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];

  let file_data = fs::read_to_string(file_path)
    .expect("Could not open file.");

  let (list_1, list_2) = read_vectors(file_data);

  let total_distance = calculate_differences(list_1.clone(), list_2.clone());
  let similar_score = calculate_sim(list_1, list_2);

  println!("Total Distance: {total_distance}");
  println!("Similarity Score: {similar_score}");
}
