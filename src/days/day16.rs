use std::collections::{vec_deque, HashMap, HashSet, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till},
    character::complete::{alpha1, digit1},
    combinator::{into, map, map_res},
    multi::many1,
    sequence::{pair, preceded, terminated},
    IResult,
};

pub struct Valve<'a> {
    index: &'a str,
    flow_rate: i32,
    connections: Vec<&'a str>,
}

pub struct Input<'a> {
    map: HashMap<&'a str, Valve<'a>>,
}

fn parse_connections(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((terminated(alpha1, tag(", ")), alpha1)))(input)
}

fn parse_line(input: &str) -> IResult<&str, Valve> {
    // let (_, ((x1, y1), (x2, y2))) = pair(parse_pair, parse_pair)(input)?;

    // let sensor = (x1, y1);
    // let beacon = (x2, y2);

    let into_i32 = map(digit1, |s: &str| s.parse::<i32>());

    let (input, index) = preceded(tag("Valve "), alpha1)(input)?;
    let (input, flow_rate) = preceded(tag(" has flow rate="), into_i32)(input)?;
    let flow_rate = flow_rate.unwrap();
    let (_, connections) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        parse_connections,
    )(input)?;

    let valve = Valve {
        index,
        flow_rate,
        connections,
    };
    Ok((input, valve))
}

fn best_choice_bfs<'a>(
    input: &Input<'a>,
    curr_node: &'a str,
    moves_left: i32,
    opened: &HashSet<&str>,
) -> (&'a str, i32, i32) {
    let mut to_search: VecDeque<(&str, i32)> = VecDeque::new();
    to_search.push_back((curr_node, 0));
    let mut seen: HashSet<&str> = HashSet::new();

    let mut best = 0;
    let mut best_str = "";
    let mut moves_taken = 0;
    while let Some((curr, depth)) = to_search.pop_front() {
        let node = input.map.get(curr).unwrap();
        for idx in &node.connections {
            if !seen.contains(idx) {
                to_search.push_back((idx, depth + 1));
                seen.insert(idx);
            }
        }

        // It takes an extra move to open the valve
        if !opened.contains(curr) {
            let value = node.flow_rate * (moves_left - depth - 1);
            if value > best {
                best = value;
                best_str = curr;
                moves_taken = depth + 1;
            }
        }
    }

    (best_str, best, moves_taken)
}

pub fn generator(input: &str) -> Input {
    let lines = input.lines();
    let mut map: HashMap<&str, Valve> = HashMap::new();
    for line in lines {
        if let Ok((_, valve)) = parse_line(line) {
            map.insert(valve.index, valve);
        } else {
            panic!();
        }
    }

    Input { map }
}

pub fn part1(input: &Input) -> i32 {
    let mut moves = 30;
    let mut sum = 0;
    let mut opened: HashSet<&str> = HashSet::new();
    let mut curr_location = "AA";

    while moves > 0 {
        println!("Running:");
        println!(
            "  Sum: {}, Moves: {}, Curr: {:?}",
            sum, moves, curr_location
        );
        println!("  Opened: {:?}", opened);
        // Evaluate all unopened valves and greedily decide what to open in the moment
        let (idx, score, moved) = best_choice_bfs(input, curr_location, moves, &opened);

        sum += score;
        moves -= moved;

        curr_location = idx;
        opened.insert(idx);

        if score == 0 {
            moves -= 1;
        }
    }

    sum
}

pub fn part2(input: &Input) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_example() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        let input = generator(input);

        let valve = input.map.get("BB").unwrap();
        let expected_valve = Valve {
            index: "BB",
            flow_rate: 13,
            connections: vec!["CC", "AA"],
        };
        assert_eq!(valve.index, expected_valve.index);
        assert_eq!(valve.flow_rate, expected_valve.flow_rate);
        assert_eq!(valve.connections, expected_valve.connections);

        let result = part1(&input);
        assert_eq!(result, 1651);

        let result = part2(&input);
        assert_eq!(result, -1);
    }
}
