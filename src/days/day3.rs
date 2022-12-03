
pub fn generator(input: &str) -> Vec<&str> {
    // let mut values = Vec::new();
    input.lines().into_iter().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    let mut sum = 0;
    for &line in input {
        let n = line.len()/2;
        let bytes = line.as_bytes();

        let (a, b) = (&bytes[0..n], &bytes[n..n*2]);

        let &c = a.iter().filter(|c| b.contains(c)).next().unwrap();
        sum += match c as char {
            'a'..='z' => c - 'a' as u8 + 1,
            'A'..='Z' => c - 'A' as u8 + 27,
            _ => unreachable!()
        } as u32;
    }
    sum
}

pub fn part2(input: &[&str]) -> u32 {
    let mut sum = 0;
    let mut n = input.len();
    let mut iter = input.iter();
    while n != 0 {
        let (a, b, c) = (iter.next().unwrap().as_bytes(), iter.next().unwrap().as_bytes(), iter.next().unwrap().as_bytes());

        let &ch = a.iter().filter(|ch| b.contains(ch) && c.contains(ch)).next().unwrap();

        sum += match ch as char {
            'a'..='z' => ch - 'a' as u8 + 1,
            'A'..='Z' => ch - 'A' as u8 + 27,
            _ => unreachable!()
        } as u32;

        n -= 3;
    }
    sum
}
