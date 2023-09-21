
pub fn generator(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut cycles: i32 = 1;
    let mut counter: i32 = 21;
    let mut reg: i32 = 1;

    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        match split[..] {
            ["noop", ..] => {
                // noop instruction
                if counter == 40 {
                    sum = sum + reg * cycles;
                }
                cycles = cycles + 1;
                counter = (counter % 40) + 1;
            },
            ["addx", val] => {
                // addx instruction
                if counter == 39 {
                    sum = sum + reg * (cycles + 1);
                }
                if counter == 40 {
                    sum = sum + reg * cycles;
                }
                cycles = cycles + 2;
                counter = ((counter + 1) % 40) + 1;
                let v: i32 = val.parse().unwrap();
                reg = reg + v;
            },
            _ => {

            },
        }
    }

    sum
}

pub fn part2(input: &str) -> i32 {
    let mut pos: i32 = 0;
    let mut cycles: i32 = 1;
    let mut reg: i32 = 1;

    let mut vec = Vec::<char>::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        match split[..] {
            ["noop", ..] => {
                // noop instruction
                if reg - 1 <= pos && pos <= reg + 1 {
                    vec.push('#');
                } else {
                    vec.push('.');
                }
                pos = (pos + 1) % 40;
                cycles = cycles + 1;
            },
            ["addx", val] => {
                // addx instruction
                if reg - 1 <= pos && pos <= reg + 1 {
                    vec.push('#');
                } else {
                    vec.push('.');
                }
                pos = (pos + 1) % 40;

                if reg - 1 <= pos && pos <= reg + 1 {
                    vec.push('#');
                } else {
                    vec.push('.');
                }
                pos = (pos + 1) % 40;

                cycles = cycles + 2;
                let v: i32 = val.parse().unwrap();
                reg = reg + v;
            },
            _ => {

            },
        }
    }
    for (i, c) in vec.iter().enumerate() {
        if i % 40 == 39 {
            println!("{}", c)
        } else {
            print!("{}", c)
        }
    }
    0
}
