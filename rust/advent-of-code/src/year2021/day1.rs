use std::i32::MAX;


pub type ParseData = i32;
pub type Input = Vec<ParseData>;

pub fn parse_input(input: String) -> Vec<ParseData> {
  input.split('\n')
    .filter_map(|s| s.to_string().parse::<ParseData>().ok())
    .collect()
}

pub fn part_one(input: &Input) {
  let mut count = 0;
  let mut prev = MAX;
  for &i in input {
    if prev < i {
      count += 1;
    }
    prev = i;
  }
  println!("Count is {}", count);
}

pub fn part_two(input: &Input) {
  let mut count = 0;
  let mut prev = input[0] + input[1] + input[2];
  for i in 3..(input.len()) {
    let sum = input[i] + input[i-1] + input[i-2];
    if prev < sum {
      count += 1;
    }
    prev = sum;
  }
  println!("Count is {}", count);
}
