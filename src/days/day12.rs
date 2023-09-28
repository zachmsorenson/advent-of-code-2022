use std::{collections::{HashMap, HashSet, BinaryHeap}, cmp::Ordering};
use std::cell::RefCell;


type Point = (i32, i32);
type PointHeight = (i32, i32, i32);

#[derive(Eq)]
struct PointScore {
    point: PointHeight,
    score: i32,
}

impl Ord for PointScore {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Less => {
                Ordering::Greater
            },
            Ordering::Equal => {
                Ordering::Equal
            },
            Ordering::Greater => {
                Ordering::Less
            },
        }
    }
}

impl PartialOrd for PointScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PointScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

struct Node {
    height: i32,
    f: i32,
    g: i32,
    h: i32,
}

pub struct Grid {
    grid: HashMap<Point, i32>,
    start: PointHeight,
    end: PointHeight,
    rows: i32,
    cols: i32,
}

fn calc_dist(src: PointHeight, dst: PointHeight) -> i32 {
    i32::abs(src.0 - dst.0) + i32::abs(src.1 - dst.1)
}

pub fn generator(input: &str) -> Grid {
    let mut start: PointHeight = (-1, -1, -1);
    let mut end: PointHeight = (-1, -1, -1);

    let cols = input.find("\n").unwrap() as i32;
    let rows = input.lines().count() as i32;
    let mut grid: HashMap<Point, i32> = HashMap::new();

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let coords: Point = (i as i32, j as i32);
            match c {
                'S' => {
                    grid.insert(coords, 0);
                    start = (coords.0, coords.1, 0);
                },
                'a'..='z' => {
                    grid.insert(coords, c as i32 - 'a' as i32 + 1 as i32);
                },
                'E' => {
                    let height = 'z' as i32 - 'a' as i32 + 2;
                    grid.insert(coords, height);
                    end = (coords.0, coords.1, height);
                },
                _ => unreachable!(),
            };

        }
    }

    let grid = Grid {
        grid,
        start,
        end,
        rows,
        cols,
    };
    grid
}

pub fn part1(input: &Grid) -> i32 {
    let mut to_search: BinaryHeap<PointScore> = BinaryHeap::new();
    let mut closed_set: HashSet<Point> = HashSet::new();
    
    let mut node_map: HashMap<Point, RefCell<Node>> = HashMap::new();
    for (point, height) in &input.grid {
        let mut new_node = Node {
            height: *height,
            f: i32::MAX,
            g: i32::MAX,
            h: i32::MAX,
        };
        if *point == (input.start.0, input.start.1) {
            new_node.f = 0;
            new_node.g = 0;
            new_node.h = 0;
        }
        node_map.insert(*point, RefCell::new(new_node));
    }

    let offsets = vec![(1, 0), (0, 1), (0, -1), (-1, 0)];

    let start_point: PointScore = PointScore {
        point: input.start,
        score: 0,
    };
    to_search.push(start_point);
    
    let end_point: PointHeight = input.end;

    while !to_search.is_empty() {
        let point_score = to_search.pop().unwrap();
        let curr_point = (point_score.point.0, point_score.point.1);
        let curr_node = node_map.get(&curr_point).unwrap().borrow();

        let neighbors = offsets.iter()
                               .map(|offset| (curr_point.0 + offset.0, curr_point.1 + offset.1))
                               .filter(|(i, j)| 0 <= *i && *i < input.cols && 0 <= *j && *j < input.rows);
         
        for neighbor_point in neighbors {
            let mut neighbor_node = node_map.get(&neighbor_point).unwrap().borrow_mut();


            if neighbor_node.height > curr_node.height + 1 {
                // Can't travel here if height difference > 1
                continue
            }

            if neighbor_point == (end_point.0, end_point.1) {
                // Stop condition
                return point_score.score;
            }

            let new_g = curr_node.g + 1;
            let new_h = calc_dist(end_point, (neighbor_point.0, neighbor_point.1, 0));
            let new_f = new_g + new_h;

            if neighbor_node.f <= new_f {
                continue;
            }

            neighbor_node.g = new_g;
            neighbor_node.h = new_h;
            neighbor_node.f = new_f;

            let new_point_score = PointScore {
                point: (neighbor_point.0, neighbor_point.1, neighbor_node.height),
                score: new_f,
            };
            to_search.push(new_point_score);
        }

        closed_set.insert(curr_point);
    }
    -1
}

pub fn part2(input: &Grid) -> i32 {
    /* Use Dijkstra's algorithm starting at the end point.
    *  We can stop at the first 'a' height node we find.
    */
    let mut to_search: BinaryHeap<PointScore> = BinaryHeap::new();
    let mut closed_set: HashSet<Point> = HashSet::new();
    
    // Let's simply use f as the number of steps taken from the summit as the score.
    let mut node_map: HashMap<Point, RefCell<Node>> = HashMap::new();
    for (point, height) in &input.grid {
        let mut new_node = Node {
            height: *height,
            f: i32::MAX,
            g: i32::MAX,
            h: i32::MAX,
        };
        if *point == (input.end.0, input.end.1) {
            new_node.f = 0;
            new_node.g = 0;
            new_node.h = 0;
        }
        node_map.insert(*point, RefCell::new(new_node));
    }

    let offsets = vec![(1, 0), (0, 1), (0, -1), (-1, 0)];

    let start_point: PointScore = PointScore {
        point: input.end,
        score: 0,
    };
    to_search.push(start_point);
    
    let mut stop_val: i32 = -1;

    let mut found_end = false;
    while !to_search.is_empty() && !found_end {
        let point_score = to_search.pop().unwrap();
        let curr_point = (point_score.point.0, point_score.point.1);
        let curr_node = node_map.get(&curr_point).unwrap().borrow();

        let neighbors = offsets.iter()
                               .map(|offset| (curr_point.0 + offset.0, curr_point.1 + offset.1))
                               .filter(|(i, j)| 0 <= *i && *i < input.cols && 0 <= *j && *j < input.rows)
                               .filter(|point| !closed_set.contains(point));
         
        for neighbor_point in neighbors {
            let mut neighbor_node = node_map.get(&neighbor_point).unwrap().borrow_mut();

            if neighbor_node.height < curr_node.height - 1 {
                // Can't travel here if height difference > 1
                continue
            }

            if neighbor_node.height == 1 {
                // Stop condition
                stop_val = point_score.score + 1;
                found_end = true;
                break;
            }

            let new_f = point_score.score + 1;

            if neighbor_node.f <= new_f {
                continue;
            }

            neighbor_node.f = new_f;

            let new_point_score = PointScore {
                point: (neighbor_point.0, neighbor_point.1, neighbor_node.height),
                score: new_f,
            };
            to_search.push(new_point_score);
        }

        closed_set.insert(curr_point);
    }
    
    stop_val
}

