use std::collections::{HashSet};


pub fn generator(input: &str) -> Vec<(i32, i32)> {
    input.lines().map(|line| {
        let mut dir = (0, 0);
        let mut value = 0;

        for c in line.bytes().into_iter() {
            match c {
                b'U' => { dir = (0, 1) },
                b'D' => { dir = (0, -1) },
                b'L' => { dir = (-1, 0) },
                b'R' => { dir = (1, 0) },
                b'0'..=b'9' => { value = value * 10 + (c - b'0') as i32 }
                _ => {}
            }
        }
        (dir.0 * value, dir.1 * value)
    }).collect()
}

fn implementation(input: &[(i32, i32)], part2: bool) -> u32 {
    // use (x, y) coordinates to track position, starting at (0, 0)
    let mut knots: Vec<(i32, i32)>;
    if part2 {
        knots = vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
    } else {
        knots = vec![
            (0, 0),
            (0, 0),
        ]
    }
    let n = knots.len();

    // track points the tail visited using a hashset
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut sum = 1;

    for &mv in input {
        let mut value;
        let sign;
        let unit_move = match mv {
            (x, 0) if x > 0 => {
                sign = x.signum();
                value = sign * x;
                (1, 0)
            },
            (x, 0) if x < 0 => {
                sign = x.signum();
                value = sign * x;
                (-1, 0)
            },
            (0, y) if y > 0 => {
                sign = y.signum();
                value = sign * y;
                (0, 1)
            },
            (0, y) if y < 0 => {
                sign = y.signum();
                value = sign * y;
                (0, -1)
            },
            _ => unreachable!()
        };

        while value != 0 {
            for i in 0..knots.len() {
                if i == 0 {
                    let mut knot: &mut (i32, i32) = &mut knots[i];
                    knot.0 += unit_move.0;
                    knot.1 += unit_move.1;
                } else {
                    let (left, right) = knots.split_at_mut(i);
                    let knot = &mut right[0];
                    let prev = left[i-1];
                    // let (knot, prev) = &mut (knots[i], knots[i-1]);
                    let diff: (i32, i32)  = (prev.0 - knot.0, prev.1 - knot.1);
                    if diff.0 > 1 || diff.0 < -1 || diff.1 > 1 || diff.1 < -1 {
                        knot.0 += diff.0.signum();
                        knot.1 += diff.1.signum();

                        // tail moved
                        if i == n - 1 {
                            if !visited.contains(knot) {
                                visited.insert(*knot);
                                sum += 1;
                            }
                        }
                    } else {
                        // if a knot doesn't move, the rest behind it won't
                        break;
                    }
                }
            }
            value -= 1;
        }
    }
    sum
}

pub fn part1(input: &[(i32, i32)]) -> u32 {
    implementation(input, false)
}

pub fn part2(input: &[(i32, i32)]) -> u32 {
    implementation(input, true)
}
