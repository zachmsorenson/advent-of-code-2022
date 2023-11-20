type Point = (usize, usize);

pub struct Ring<'a, T> {
    data: &'a Vec<T>,
    cursor: usize,
    length: usize,
}

impl<'a, T> Ring<'a, T> {
    pub fn from_vec(v: &Vec<T>) -> Ring<T> {
        Ring {
            data: v,
            cursor: 0,
            length: v.len(),
        }
    }

    pub fn get_next(&mut self) -> &T {
        let item = &self.data[self.cursor];
        if self.cursor == self.length - 1 {
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }
        item
    }
}

pub enum Block {
    Row,
    Cross,
    L,
    Col,
    Square,
}

impl Block {
    pub fn as_points(&self) -> Vec<Point> {
        match self {
            Block::Row => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Block::Cross => vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
            Block::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Block::Col => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Block::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    /// height indicates the height of the current stack.
    /// blocks spawn 3 units above the current stack and 3 units from the left edge.
    /// I count walls explicitly here so the bottom empty row is y = 1.
    pub fn spawn_points(&self, height: usize) -> Vec<Point> {
        let mut points = self.as_points();
        Self::translate(&mut points, 3, height as i32 + 4);
        points
    }

    pub fn translate(points: &mut Vec<Point>, x: i32, y: i32) {
        for point in points {
            *point = ((point.0 as i32 + x) as usize, (point.1 as i32 + y) as usize);
        }
    }
}

#[derive(Clone, Debug)]
pub enum Space {
    Empty,
    Filled,
}

pub struct Field {
    data: Vec<Space>,
    width: usize,
    height: usize,
}

impl Field {
    pub fn new(width: usize) -> Field {
        let data = vec![Space::Filled; width + 2];
        Field {
            data,
            width: width + 2,
            height: 0,
        }
    }

    fn at(&self, x: usize, y: usize) -> &Space {
        if x == 0 || x > self.width - 2 {
            return &Space::Filled;
        }
        let idx = y * self.width + x;
        if idx > self.data.len() {
            return &Space::Empty;
        }
        &self.data[idx]
    }

    pub fn check_collides(&self, points: &Vec<Point>, x: i32, y: i32) -> bool {
        for &(px, py) in points {
            let (x, y) = (px as i32 + x, py as i32 + y);
            match self.at(x as usize, y as usize) {
                Space::Filled => return true,
                _ => continue,
            }
        }
        false
    }

    pub fn set_points(&mut self, points: &Vec<Point>) {
        for &(px, py) in points {
            while self.height < py {
                let mut v = vec![Space::Empty; self.width];
                v[0] = Space::Filled;
                v[self.width - 1] = Space::Filled;
                self.data.append(&mut v);
                self.height += 1;
            }

            self.data[py * (self.width) + px] = Space::Filled;
        }
    }

    pub fn print(&self) {
        for y in (0..=self.height).rev() {
            let mut s = "".to_string();
            for x in 0..self.width {
                let c = match self.at(x, y) {
                    Space::Empty => '.',
                    Space::Filled => '#',
                };
                s.push(c);
            }
            println!("{:?}", s);
        }
    }
}

pub struct Input {
    directions: Vec<i32>,
    blocks: Vec<Block>,
}

pub fn generator(input: &str) -> Input {
    let mut directions: Vec<i32> = Vec::new();
    for c in input.chars() {
        let dir = match c {
            '<' => -1,
            '>' => 1,
            _ => break,
        };
        directions.push(dir);
    }

    let blocks = vec![
        Block::Row,
        Block::Cross,
        Block::L,
        Block::Col,
        Block::Square,
    ];

    Input { directions, blocks }
}

pub fn part1(input: &Input) -> i32 {
    let mut directions = Ring::from_vec(&input.directions);
    let mut blocks = Ring::from_vec(&input.blocks);

    let mut field = Field::new(7);

    for i in 0..2022 {
        let block = blocks.get_next();
        let mut points = block.spawn_points(field.height);

        let mut falling = true;
        while falling {
            let next_direction = directions.get_next();

            // push to the side
            if !field.check_collides(&points, *next_direction, 0) {
                Block::translate(&mut points, *next_direction, 0);
            }

            // fall one
            if field.check_collides(&points, 0, -1) {
                field.set_points(&points);
                falling = false;
            } else {
                Block::translate(&mut points, 0, -1);
            }
        }
    }

    field.height as i32
}

pub fn part2(input: &Input) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let input = generator(input);

        let output = part1(&input);

        assert_eq!(output, 3068)
    }
}
