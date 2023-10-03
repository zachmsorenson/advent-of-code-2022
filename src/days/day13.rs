

#[derive(Debug, PartialEq)]
pub enum Item {
    Value(i32),
    Open,
    Close,
}

pub fn generator(input: &str) -> Vec<Vec<Item>> {
    let mut items = Vec::<Vec<Item>>::new();

    for line in input.lines().filter(|&l| l != "") {
        let mut packet = Vec::<Item>::new();
        let mut value = 0;
        let mut parse = false;
        for c in line.chars() {
            macro_rules! cond_append_value {
                ($cond:expr) => {
                    if ($cond) {
                        packet.push(Item::Value(value));
                        parse = false;
                        value = 0;
                    }
                }
            }
            match c {
                '[' => {
                    cond_append_value!(parse);
                    packet.push(Item::Open)
                },
                ']' => {
                    cond_append_value!(parse);
                    packet.push(Item::Close)
                },
                ',' => {
                    cond_append_value!(parse);
                },
                ('0'..='9') => {
                    value = 10 * value + (c as i32 - '0' as i32);
                    parse = true;
                },
                _ => panic!()
            };
        }
        items.push(packet);
    }

    items
}

pub fn part1(input: &Vec<Vec<Item>>) -> i32 {
    let mut i = 1;
    let mut sum = 0;
    let mut packets = input.iter();
    while let Some((l_packet, r_packet)) = packets.next().zip(packets.next()) {
        let mut l_iter = l_packet.iter();
        let mut r_iter = r_packet.iter();
        let mut l_depth = 0;
        let mut r_depth = 0;

        let mut in_order = None;
        let mut l_next = l_iter.next();
        let mut r_next = r_iter.next();
        while l_next.is_some() && r_next.is_some() {
            let l = l_next.unwrap();
            let r = r_next.unwrap();
            match (l, r) {
                (Item::Open, Item::Open) => {
                    l_depth += 1;
                    r_depth += 1;
                    l_next = l_iter.next();
                    r_next = r_iter.next();
                },
                (Item::Close, Item::Close) => {
                    l_depth -= 1;
                    r_depth -= 1;
                    l_next = l_iter.next();
                    r_next = r_iter.next();
                },
                (Item::Value(l_val), Item::Value(r_val)) => {
                    if l_val < r_val {
                        in_order = Some(true);
                        // println!("Break at {} < {}, result: {}", l_val, r_val, in_order.unwrap());
                        break;
                    } else if r_val < l_val {
                        in_order = Some(false);
                        // println!("Break at {} > {}, result: {}", l_val, r_val, in_order.unwrap());
                        break;
                    } else if l_depth != r_depth {
                        in_order = Some(l_depth < r_depth);
                        break;
                    } else {
                        l_next = l_iter.next();
                        r_next = r_iter.next();
                    }
                },
                (Item::Close, x @ _) => {
                    // Left side ran out of items.
                    if l_depth == r_depth {
                        in_order = Some(true);
                        // println!("Break at ] vs {:?}, result: {}", x, in_order.unwrap());
                        break;
                    } else {
                        l_depth -= 1;
                        l_next = l_iter.next();
                    }
                },
                (x @ _, Item::Close) => {
                    // Right side ran out of items.
                    if l_depth == r_depth {
                        in_order = Some(false);
                        // println!("Break at {:?} vs ], result: {}", x, in_order.unwrap());
                        break;
                    } else {
                        r_depth -= 1;
                        r_next = r_iter.next();
                    }
                },
                (Item::Open, Item::Value(_)) => {
                    l_depth += 1;
                    l_next = l_iter.next();
                },
                (Item::Value(_), Item::Open) => {
                    r_depth += 1;
                    r_next = r_iter.next();
                },
            }
        }
        if in_order.is_some() && in_order.unwrap() == true {
            // println!("{}", i);
            sum += i;
        } else if in_order.is_none() {
            panic!("No break at pair {}", i);
        }
        // println!("Pair[{}]: {}  |  sum: {}", i, in_order.unwrap(), sum);
        // println!("l_packet: {:?}", l_packet);
        // println!("r_packet: {:?}", r_packet);
        // println!("");
        i += 1;
    }
    return sum;
}

pub fn part2(input: &Vec<Vec<Item>>) -> i32 {
    -1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let input = "[1,[2]]\n[1]\n[]";
        let result = generator(input);
        let mut iter = result.iter();

        let result1 = iter.next().unwrap();
        let result2 = iter.next().unwrap();
        let expected1 = vec![Item::Open, Item::Value(1), Item::Open, Item::Value(2), Item::Close, Item::Close];
        let expected2 = vec![Item::Open, Item::Value(1), Item::Close];
        
        assert_eq!(result1, &expected1, "expect parse success1: result {:?} vs expected {:?}", result1, expected1);
        assert_eq!(result2, &expected2, "expect parse success2: result {:?} vs expected {:?}", result2, expected2);
    }

    #[test]
    fn test_part1_bad() {
//         let input = "[[[[6,6,1],[3,10,5]],[[3,9,6,0],2,[10,1,8,8]],[],[[6,9],5,5,[],4]],[[7],3],[[[],[8,4,2,0,8]],10,2,7],[8],[10,7,[[2,6,0,0,8],[3,10,10],[],9,[7,9,3]],6]]
// [[6,8,[[9,0,8,7,5],0,3],[],0],[8,[6,9],8,[[0],7,[10,6],6,[7]],8]]
//
// ";
        let input = "[[6, 6, 1]]
[6, 8]
";
        let input = generator(input);

        let result = part1(&input);
        let expected = 0;

        assert_eq!(result, expected, "Supplied test case: [actual]: {}, [expected]: {}", result, expected);

    }

    #[test]
    fn test_part1_small() {
        let input = "
[1,2,3,4]
[1,2,3]

[1,2,3]
[1,2,3,4]

[1,2,3]
[1,2,5]

[1,2,5]
[1,2,3]
";
        let input = generator(input);

        let result = part1(&input);
        let expected = 5;

        assert_eq!(result, expected, "Supplied test case: [actual]: {}, [expected]: {}", result, expected);

    }

    #[test]
    fn test_part1_full() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let input = generator(input);

        let result = part1(&input);
        let expected = 13;

        assert_eq!(result, expected, "Supplied test case: [actual]: {}, [expected]: {}", result, expected);
    }
}

