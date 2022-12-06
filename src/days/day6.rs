use std::collections::HashMap;
use std::collections::VecDeque;


pub fn generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

/// Implemented with a VecDeque
pub fn part1(input: &[char]) -> u32 {
    let mut deque = VecDeque::with_capacity(5);
    for i in 0..input.len() {
        let next = input[i];
        deque.push_back(next);

        if i >= 3 {
            let (a, b, c, d) = (deque[0], deque[1], deque[2], next);
            if a != b && a != c && a != d && b != c && b != d && c != d {
                return i as u32 + 1;
            }

            deque.pop_front().unwrap();
        }
    }
    0
}

/// Implemented with a HashMap
pub fn part2(input: &[char]) -> u32 {
    let mut hashmap: HashMap<char, u32> = HashMap::with_capacity(26);

    for c in 'a'..='z' {
        hashmap.insert(c, 0);
    }

    for i in 0..input.len() {
        let next = input[i];
        *hashmap.get_mut(&next).unwrap() += 1;

        if i >= 13 {
            if hashmap.values().all(|&v| v <= 1) {
                return i as u32 + 1;
            }

            let lru = input[i-13];
            *hashmap.get_mut(&lru).unwrap() -= 1;
        }
    }
    0
}
