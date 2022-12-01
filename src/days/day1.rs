
pub type Int = i32;

/// Parse each line to an integer.
pub fn generator(input: &str) -> Vec<Int> {
    let mut values = Vec::new();
    let mut count = 0;
    for line in input.lines().into_iter() {
        match line.parse::<Int>() {
            Ok(v) => count += v,
            Err(_) => {
                values.push(count);
                count = 0
            }
        };
    }
    values.sort_by(|a, b| b.cmp(a));
    values
}

pub fn part1(input: &[Int]) -> Int {
    input[0]
}

pub fn part2(input: &[Int]) -> Int {
    input[0] + input[1] + input[2]
}
