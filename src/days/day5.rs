
#[derive(Clone)]
pub struct Input {
    stacks: Vec<Vec<char>>,
    instructions: Vec<usize>
}

impl Input {
    fn init(instructions: Vec<usize>) -> Input {
        let mut input = Input {
            stacks: vec![],
            instructions: instructions,
        };
        input.reset();
        input
    }

    fn reset(&mut self) {
        self.stacks = vec![
            vec!['D', 'T', 'R', 'B', 'J', 'L', 'W', 'G'],
            vec!['S', 'W', 'C'],
            vec!['R', 'Z', 'T', 'M'],
            vec!['D', 'T', 'C', 'H', 'S', 'P', 'V'],
            vec!['G', 'P', 'T', 'L', 'D', 'Z'],
            vec!['F', 'B', 'R', 'Z', 'J', 'Q', 'C', 'D'],
            vec!['S', 'B', 'D', 'J', 'M', 'F', 'T', 'R'],
            vec!['L', 'H', 'R', 'B', 'T', 'V', 'M'],
            vec!['Q', 'P', 'D', 'S', 'V'],
        ]
    }
}

pub fn generator(input: &str) -> Input {
    let chars = input.chars().into_iter().skip(325);
    let mut instructions = Vec::<usize>::new();
    let mut num: usize = 0;
    for c in chars {
        match c {
            '0'..='9' => {
                num = num * 10 + c as usize - b'0' as usize;
            },
            _ => {
                if num > 0 {
                    instructions.push(num);
                    num = 0;
                }
            },
        }
    }
    instructions.push(num);

    return Input::init(instructions);
}

pub fn part1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    let instructions = &input.instructions;
    let n = input.instructions.len();
    let mut i = 0;
    while i < n {
        let mut amount = instructions[i];
        let src = instructions[i + 1] - 1;
        let dst = instructions[i + 2] - 1;

        while amount > 0 {
            match stacks[src].pop() {
                Some(c) => {
                    stacks[dst].push(c);
                },
                None => unreachable!()
            };
            amount -= 1;
        }

        i += 3;
    }

    return stacks.iter().map(|s| s.last().unwrap()).cloned().collect()
}

pub fn part2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    let instructions = &input.instructions;
    let n = input.instructions.len();
    let mut i = 0;
    while i < n {
        let amount = instructions[i];
        let src = instructions[i + 1] - 1;
        let dst = instructions[i + 2] - 1;

        let src_n = stacks[src].len();
        let src_slice = stacks[src].drain(src_n - amount..);
        let mut new: Vec<char> = src_slice.collect();

        stacks[dst].append(&mut new);

        i += 3;
    }
    
    return stacks.iter().map(|s| s.last().unwrap()).cloned().collect()
}
