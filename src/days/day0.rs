
use std::u32::MAX;

pub type Int = u32;

/// Parse each line to an integer.
pub fn generator(input: &str) -> Vec<Int> {
    input
        .lines()
        .map(|x| x.parse().expect("Not an integer"))
        .collect()
}

pub fn part1(input: &[Int]) -> Int {
    let mut count = 0;
    let mut prev = MAX;
    for &i in input {
        if prev < i {
        count += 1;
        }
        prev = i;
    } 
    count
}


pub fn part2(input: &[Int]) -> Int {
    let mut count = 0;
    let mut prev = input[0] + input[1] + input[2];
    for (i, _) in input.iter().skip(2).enumerate() {
        let sum = input[i+2] + input[i+1] + input[i];
        if prev < sum {
        count += 1;
        }
        prev = sum;
    } 
    count
}
