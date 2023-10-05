

type Point = (i32, i32);

#[derive(Clone, Copy)]
pub enum Fill {
    Empty,
    Rock,
    Sand,
    Abyss,
}

#[derive(Clone)]
pub struct Grid {
    array: Vec<Fill>,
    height: i32,
    width: i32,
    left_edge: i32,
}

impl Grid {
    fn get(&self, p: Point) -> Fill {
        if p.0 < 0 || p.0 >= self.width || p.1 >= self.height {
            return Fill::Abyss;
        }
        let idx = (p.0 * self.height + p.1) as usize;
        return self.array[idx];
    }

    fn set(&mut self, p: Point) {
        let idx = (p.0 * self.height + p.1) as usize;
        self.array[idx] = Fill::Sand;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get((x, y)) {
                    Fill::Empty => print!("."),
                    Fill::Rock => print!("#"),
                    Fill::Sand => print!("o"),
                    Fill::Abyss => print!("X"),
                };
            }
            println!("")
        }
    }
}

pub fn generator(input: &str) -> Grid {
    let mut features: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut x_min = i32::MAX;
    let mut x_max = 0;
    let mut y_max = 0;

    for line in input.lines() {
        let pairs: Vec<(i32, i32)> = line.split(" -> ")
            .map(|p| {
                let mut pair = p.split(',');
                let x: i32 = pair.next().unwrap().parse().expect("x input should parse to i32");
                let y: i32 = pair.next().unwrap().parse().expect("y input should parse to i32");
                x_min = i32::min(x_min, x);
                x_max = i32::max(x_max, x);
                y_max = i32::max(y_max, y);
                return (x, y);
            })
            .collect();
        features.push(pairs);
    };

    let width = x_max - x_min + 1;
    let height = y_max + 1;

    // let mut grid = Vec::<Fill>::with_capacity((width * height) as usize);
    let mut grid = Vec::<Fill>::new();
    grid.resize((width * height) as usize, Fill::Empty);

    for points in features {
        let mut points = points.iter().map(|p| (p.0 - x_min, p.1));
        let mut curr: Point = points.next().unwrap();
        let unit: Point = (0, 0);
        for next in points {
            let diff: Point = (curr.0 - next.0, curr.1 - next.1);
            let unit = match diff {
                (..=-1, 0) => (1, 0),
                (1.., 0) => (-1, 0),
                (0, 1..) => (0, -1),
                (0, ..=-1) => (0, 1),
                (_, _) => unreachable!(),
            };
            grid[(curr.0 * height + curr.1) as usize] = Fill::Rock;
            while curr != next {
                curr = (curr.0 + unit.0, curr.1 + unit.1);
                grid[(curr.0 * height + curr.1) as usize] = Fill::Rock;
            }
        }
    }

    let grid = Grid {
        array: grid,
        height,
        width,
        left_edge: x_min,
    };

    return grid;
}

pub fn part1(input: &Grid) -> i32 {
    let mut count = 0;
    let mut grid: Grid = input.clone();
    let start: Point = (500 - input.left_edge, 0);
    let mut pos_stack = vec![start];

    let mut running = true;
    while running {
        // start next grain of sand from previous position of last
        let mut curr_pos = *pos_stack.last().unwrap();
        let mut settled = true;

        for unit in [(0, 1), (-1, 1), (1, 1)] {
            let check_pos = (curr_pos.0 + unit.0, curr_pos.1 + unit.1);
            let check_fill = grid.get(check_pos);
            match check_fill {
                Fill::Empty => {
                    settled = false;
                    curr_pos = check_pos;
                    grid.set(check_pos);
                    break;
                },
                Fill::Rock | Fill::Sand => {
                    continue;
                },
                // expect to find a settle pos on first fall// expect to find a settle pos on first fall
                Fill::Abyss => {
                    running = false;
                },
            }
        }

        if running && settled {
            count += 1;
            pos_stack.pop();
        } else if running {
            pos_stack.push(curr_pos);
        }
    }

    return count;
}

pub fn part2(input: &Grid) -> i32 {
    let new_height = input.height + 2;
    let new_width = new_height * 2 + 1; // centered on (500, 0)
    let new_left_edge = 500 - new_height;
    let mut big_grid_vec = Vec::<Fill>::new();
    big_grid_vec.resize((new_width * new_height) as usize, Fill::Empty);

    let x_shift = input.left_edge - new_left_edge;
    for x in 0..input.width {
        for y in 0..input.height {
            match input.get((x, y)) {
                Fill::Rock => {
                    let shifted_x = x + x_shift;
                    big_grid_vec[(shifted_x * new_height + y) as usize] = Fill::Rock;
                },
                _ => (),
            }
        }
    }
    for x in 0..new_width {
        big_grid_vec[(x * new_height + new_height - 1) as usize] = Fill::Rock;
    }

    let mut grid = Grid {
        array: big_grid_vec,
        height: new_height,
        width: new_width,
        left_edge: new_left_edge,
    };

    let mut count = 0;
    let start: Point = (500 - grid.left_edge, 0);
    let mut pos_stack = vec![start];

    while let Some(curr_pos) = pos_stack.last() {
        // start next grain of sand from previous position of last
        let mut settled = true;
        let mut next_pos = *curr_pos;

        for unit in [(0, 1), (-1, 1), (1, 1)] {
            let check_pos = (curr_pos.0 + unit.0, curr_pos.1 + unit.1);
            let check_fill = grid.get(check_pos);
            match check_fill {
                Fill::Empty => {
                    settled = false;
                    next_pos = check_pos;
                    break;
                },
                Fill::Rock | Fill::Sand => {
                    continue;
                },
                Fill::Abyss => {
                    // should never fall off on part2
                    unreachable!();
                },
            }
        }

        if settled {
            count += 1;
            grid.set(next_pos);
            pos_stack.pop();
        } else {
            pos_stack.push(next_pos);
        }
    }
    
    return count;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_example() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let input = generator(input);

        assert_eq!(input.height, 10);
        assert_eq!(input.width, 10);
        assert_eq!(input.left_edge, 494);

        let result = part1(&input);
        assert_eq!(result, 24);

        let result = part2(&input);
        assert_eq!(result, 93);
    }
}
