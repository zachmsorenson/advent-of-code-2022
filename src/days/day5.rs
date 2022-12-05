
#[derive(Clone)]
pub struct Input {
    stacks: Vec<Vec<char>>,
    instructions: Vec<usize>
}

impl Input {
    fn init(instructions: Vec<usize>) -> Input {
        return Input {
            stacks: vec![
                vec!['G', 'W', 'L', 'J', 'B', 'R', 'T', 'D'],
                vec!['C', 'W', 'S'],
                vec!['M', 'T', 'Z', 'R'],
                vec!['V', 'P', 'S', 'H', 'C', 'T', 'D'],
                vec!['Z', 'D', 'L', 'T', 'P', 'G'],
                vec!['D', 'C', 'Q', 'J', 'Z', 'R', 'B', 'F'],
                vec!['R', 'T', 'F', 'M', 'J', 'D', 'B', 'S'],
                vec!['M', 'V', 'T', 'B', 'R', 'H', 'L'],
                vec!['V', 'S', 'D', 'P'],
            ],
            instructions: instructions,
        };
    }

    fn reset(&mut self) {
        self.stacks = vec![
            vec!['G', 'W', 'L', 'J', 'B', 'R', 'T', 'D'],
            vec!['C', 'W', 'S'],
            vec!['M', 'T', 'Z', 'R'],
            vec!['V', 'P', 'S', 'H', 'C', 'T', 'D'],
            vec!['Z', 'D', 'L', 'T', 'P', 'G'],
            vec!['D', 'C', 'Q', 'J', 'Z', 'R', 'B', 'F'],
            vec!['R', 'T', 'F', 'M', 'J', 'D', 'B', 'S'],
            vec!['M', 'V', 'T', 'B', 'R', 'H', 'L'],
            vec!['V', 'S', 'D', 'P'],
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
                instructions.push(num);
                num = 0;
            },
        }
    }

    return Input::init(instructions);
}

pub fn part1(input: &Input) -> Vec<char> {
    let mut stacks = &input.stacks;
    let instructions = &input.instructions;
    let n = input.instructions.len();
    let mut i = 0;
    while i < n {
        let mut amount = instructions[i];
        let src = instructions[i+1];
        let dst = instructions[i+2];

        while amount > 0 {
            let c = stacks[src-1].pop().unwrap();
            stacks[src-1].push(c);
        }
    }

    return stringify![stacks.iter().map(|s| *s.last().unwrap())];
}

pub fn part2(input: &Input) -> i32 {
    -1
}
