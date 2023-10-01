use std::collections::VecDeque;


#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
    Pow,
}

#[derive(Debug)]
pub struct Operation {
    op: Op,
    val: i64,
}

impl Operation {
    pub fn operate(&self, n: i64) -> i64 {
        let res = match self.op {
            Op::Add => self.val + n,
            Op::Mul => self.val * n,
            Op::Pow => n * n,
        };
        res
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisor: i64,
    dst_true: i64,
    dst_false: i64,
}

pub fn generator(input: &str) -> Vec::<Monkey> {
    // let items: Vec<&str> = input.split("\n\n").collect();

    let mut vec = Vec::<Monkey>::new();
        
    let mut starting_items: Vec::<i64> = Vec::<i64>::new();
    let mut op: Operation = Operation{
        op: Op::Add, 
        val: 0
    };
    let mut divisor: i64 = -1;
    let mut dst_true: i64 = -1;
    let mut dst_false: i64 = -1;

    for line in input.lines() {
        let mut split = line.trim_start().split(":");


        let mut chars = split.next().unwrap().chars();
        match chars.nth(0) {
            Some('M') => {
                continue
            },
            Some('S') => {
                let items = split.next().unwrap().split(",");
                starting_items = items.map(|s| s.trim().parse().unwrap()).collect();
            },
            Some('O') => {
                let rem = split.next().unwrap().trim().split(" ");
                let mut rem = rem.skip(3);
                let op_str = rem.next().unwrap();
                let val_str = rem.next().unwrap();
                match (op_str, val_str) {
                    ("+", val_str) =>  {
                        op = Operation{
                            op: Op::Add,
                            val: val_str.parse().unwrap(),
                            // val: BigNum::new(vec![val_str.parse().unwrap()]),
                        }
                    },
                    ("*", "old") => {
                        println!("pow function");
                        op = Operation{
                            op: Op::Pow,
                            val: 2,
                            // val: BigNum::new(vec![2]),
                        }
                    },
                    ("*", val_str) => {
                        op = Operation{
                            op: Op::Mul,
                            val: val_str.parse().unwrap(),
                            // val: BigNum::new(vec![val_str.parse().unwrap()]),
                        }
                    },
                    _ => continue,
                }
                
            },
            Some('T') => {
                divisor = split.next().unwrap()
                               .split(" ").last().expect("Divisor line should end with a number")
                               .parse().unwrap();
            },
            Some('I') => {
                let val: i64 = split.next().unwrap().split(' ').last().unwrap().parse().unwrap();
                let bool_char = chars.nth(2).unwrap();
                match bool_char {
                    't' => dst_true = val,
                    'f' => dst_false = val,
                    _ => {panic!("");}
                }
            },
            _ => {
                let new_monkey = Monkey{
                    items: starting_items,
                    operation: op,
                    divisor,
                    dst_true,
                    dst_false,
                };
                vec.push(new_monkey);
            
                starting_items = Vec::<i64>::new();
                op = Operation{
                    op: Op::Add, 
                    val: 0
                };
                divisor = -1;
                dst_true = -1;
                dst_false = -1;
            },
        }
    }
    let new_monkey = Monkey{
        items: starting_items,
        operation: op,
        divisor,
        dst_true,
        dst_false,
    };
    vec.push(new_monkey);
    vec
}

pub fn part1(input: &Vec<Monkey>) -> i64 {
    let mut monkey_counts: Vec::<i64> = Vec::new();
    let mut monkey_items: Vec::<VecDeque::<i64>> = Vec::new();
    for monkey in input {
        monkey_items.push(monkey.items.clone().into());
        monkey_counts.push(0)
    }

    let num_rounds = 20;

    for _ in 0..num_rounds {
        for (i, monkey) in input.iter().enumerate() {
            while let Some(curr_item) = monkey_items[i].pop_front() {
                let mut new_worry = monkey.operation.operate(curr_item);
                new_worry = new_worry / 3;
                let dst = match new_worry % monkey.divisor {
                    0 => monkey.dst_true,
                    _ => monkey.dst_false,
                };
                monkey_items[dst as usize].push_back(new_worry);
                monkey_counts[i] += 1;
            }
        }
    }

    monkey_counts.iter().fold([0, 0], |acc, x| {
        if *x > acc[0] {
            [*x, acc[0]]
        } else if *x > acc[1] {
            [acc[0], *x]
        } else {
            [acc[0], acc[1]]
        }
    }).iter().product()
}

pub fn part2(_input: &Vec<Monkey>) -> i64 {
    /*
    let mut monkey_counts: Vec::<i64> = Vec::new();
    let mut monkey_items: Vec::<VecDeque::<i64>> = Vec::new();
    for monkey in input {
        monkey_items.push(monkey.items.clone().into());
        monkey_counts.push(0)
    }

    let num_rounds = 10000;

    for _ in 0..num_rounds {
        for (i, monkey) in input.iter().enumerate() {
            while let Some(curr_item) = monkey_items[i].pop_front() {
                let mut new_worry = monkey.operation.operate(curr_item);
                new_worry = new_worry;
                let dst = match new_worry % monkey.divisor {
                    0 => monkey.dst_true,
                    _ => monkey.dst_false,
                };
                monkey_items[dst as usize].push_back(new_worry);
                monkey_counts[i] += 1;
            }
        }
    }

    monkey_counts.iter().fold([0, 0], |acc, x| {
        if *x > acc[0] {
            [*x, acc[0]]
        } else if *x > acc[1] {
            [acc[0], *x]
        } else {
            [acc[0], acc[1]]
        }
    }).iter().product()
    */
    -1
}


