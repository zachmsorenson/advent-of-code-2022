
pub fn generator(input: &str) -> Vec<&str> {
    let mut values = Vec::new();
    for line in input.lines().into_iter() {
        values.push(line);
    }
    values
}

pub fn part1(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|&s| match s {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => 0,
        })
        .sum()
}

pub fn part2(input: &[&str]) -> i32 {
    input
        .iter()
        .map(|&s| match s {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => 0,
        })
        .sum()
}
