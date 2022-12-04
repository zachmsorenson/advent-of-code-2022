
pub fn generator(input: &str) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();
    let mut b = input.as_bytes().into_iter();
    let mut num = 0;
    loop {
        match b.next() {
            Some(&c) => {
                match c {
                    b'0'..=b'9' => {
                        num = num * 10 + c as i32 - '0' as i32
                    },
                    b',' | b'-' | b'\n' => {
                        values.push(num);
                        num = 0;
                    },
                    _ => unreachable!()
                }
            },
            None => {
                break;
            },
        }
    }
    values
}

pub fn part1(input: &[i32]) -> u32 {
    let mut sum = 0;
    let n = input.len();
    let mut i = 0;
    while i < n {
        let (a, b, c, d) = (input[i], input[i + 1], input[i + 2], input[i + 3]);
        if (a >= c && b <= d) || (c >= a && d <= b) {
            sum += 1;
        }
        i += 4;
    }
    sum
}

pub fn part2(input: &[i32]) -> u32 {
    let mut sum = 0;
    let n = input.len();
    let mut i = 0;
    while i < n {
        let (a, b, c, d) = (input[i], input[i + 1], input[i + 2], input[i + 3]);
        if (a >= c && a <= d) || (c >= a && c <= b) || (b >= c && b <= d) || (c >= b && c <= b) {
            sum += 1;
        }
        i += 4;
    }
    sum
}
