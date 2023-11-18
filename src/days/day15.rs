use nom::{
    bytes::complete::take_till,
    character::complete::digit1,
    combinator::map_res,
    sequence::{pair, preceded},
    IResult,
};

type Point = (i32, i32);

pub struct Input {
    pairs: Vec<(Point, Point)>,
    y0: i32,
}

fn parse_to_next_int(input: &str) -> IResult<&str, i32> {
    preceded(
        take_till(|c: char| c.is_ascii_digit()),
        map_res(digit1, |s: &str| s.parse()),
    )(input)
}

fn parse_pair(input: &str) -> IResult<&str, (i32, i32)> {
    pair(parse_to_next_int, parse_to_next_int)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    let (_, ((x1, y1), (x2, y2))) = pair(parse_pair, parse_pair)(input)?;

    let sensor = (x1, y1);
    let beacon = (x2, y2);

    Ok((input, (sensor, beacon)))
}

pub fn generator(input: &str) -> Input {
    let lines = input.lines();
    let mut pairs: Vec<(Point, Point)> = Vec::new();
    for line in lines {
        if let Ok((_, (sensor, beacon))) = parse_line(line) {
            pairs.push((sensor, beacon));
        } else {
            panic!();
        }
    }

    Input { pairs, y0: 2000000 }
}

pub fn part1(input: &Input) -> i32 {
    let mut ranges = Vec::<(i32, i32)>::new();
    let y0 = input.y0;
    for &((x1, y1), (x2, y2)) in &input.pairs {
        let diff = (y0 - y1).abs();
        let dist = (y2 - y1).abs() + (x2 - x1).abs();

        if diff > dist {
            continue;
        }

        let mut low = 0;
        let mut high = 0;

        if x2 > x1 && y2 > y1 {
            // Top-right
            let opp = (x1 + x1 - x2, y2);
            low = y0 - y2 + opp.0;
            high = y2 - y0 + x2;
        } else if x2 < x1 && y2 > y1 {
            // Top-left
            let opp = (x1 + x1 - x2, y2);
            low = y0 - y2 + x2;
            high = y2 - y0 + opp.0;
        } else if x2 > x1 && y2 < y1 {
            // Bottom-right
            let opp = (x1 + x1 - x2, y2);
            low = y2 - y0 + opp.0;
            high = y0 - y2 + x2;
        } else if x2 < x1 && y2 < y1 {
            // Bottom-left
            let opp = (x1 + x1 - x2, y2);
            low = y0 - y2 + x2;
            high = y2 - y0 + opp.0;
        }

        if y2 == y0 && x2 < x1 {
            low += 1;
        } else if y2 == y0 && x1 < x2 {
            high -= 1;
        }

        ranges.push((low, high));
    }

    ranges.sort();
    println!("Ranges: {:?}", ranges);

    let mut sum = 0;
    let mut curr_start = ranges[0].0;
    let mut curr_end = ranges[0].1;
    for &(start, end) in &ranges {
        if start > curr_end {
            sum += curr_end - curr_start;
            curr_start = start;
            curr_end = end;
        } else if curr_end < end {
            curr_end = end;
        }
    }
    sum += curr_end - curr_start;

    sum
}

pub fn part2(input: &Input) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_example() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let mut input = generator(input);
        input.y0 = 10;

        assert_eq!(input.pairs.len(), 14);

        let result = part1(&input);
        assert_eq!(result, 26);

        let result = part2(&input);
        assert_eq!(result, -1);
    }

    #[test]
    fn day15_example_ex() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=74, y=3: closest beacon is at x=75, y=-8
Sensor at x=144, y=3: closest beacon is at x=154, y=15
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let mut input = generator(input);
        input.y0 = 10;

        assert_eq!(input.pairs.len(), 16);

        let result = part1(&input);
        assert_eq!(result, 26);

        let result = part2(&input);
        assert_eq!(result, -1);
    }
}
