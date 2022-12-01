use std::time::Duration;
use std::time::Instant;

pub mod days;
use days::*;

use clap::{Parser};

#[derive(Parser)]
struct Args {
    /// Day to run
    day: Option<usize>,
}

// Time the given function, returning its result and the elapsed time
fn time<T>(func: &dyn Fn() -> T) -> (Duration, T) {
    let start = Instant::now();
    let result = func();

    (start.elapsed(), result)
}



#[macro_export]
macro_rules! main {
    (
        implemented_days: [$($day:ident),+ $(,)?]
    ) => {
        const DAYS: &[&str] = &[$(stringify!($day)),*];
        const INPUTS : &[&str] = &[$(include_str!(concat!("../input/", stringify!($day), ".txt"))),*];

        fn main() {
            let args: Args = Args::parse();

            let days = match args.day {
                Some(day) => {
                    assert!(DAYS.contains(&format!("day{}", day).as_ref()), "Requested an unimplemented day");
                    vec![day]
                },
                None => DAYS.iter().map(|s| s[3..].parse().expect("Defaulting to all days")).collect()
            };

            for day in days.into_iter() {
                let module_name = format!("day{}", day);

                println!("{}", module_name);

                match module_name.as_ref() {
                    $(stringify!($day) => {
                        let data = INPUTS[day as usize];

                        let (gen_elapsed, input) = time(&|| $day::generator(&data));
                        let (p1_elapsed, p1_result) = time(&|| $day::part1(&input)); 
                        let (p2_elapsed, p2_result) = time(&|| $day::part2(&input));

                        let duration = format!("({:.2?})", gen_elapsed + p1_elapsed + p2_elapsed);
                        println!("{} {}", format!("Day {}", day), duration);
                        println!("   |Generator {}|",  format!("({:.2?})", gen_elapsed));
                        println!("   |Part 1 :  {}| Output: {}", format!("({:.2?})", p1_elapsed), p1_result);
                        println!("   |Part 2 :  {}| Output: {}", format!("({:.2?})", p2_elapsed), p2_result);

                        println!()
                    },)+
                    _ => unreachable!()
                }
            }
        }
    }
}

main! {
    implemented_days: [
        day0,
        // day1,
        // day2,
        // day3,
        // day4,
        // day5,
        // day6,
        // day7,
        // day8,
        // day9,
        // day10,
        // day11,
        // day12,
        // day13,
        // day14,
        // day15,
        // day16,
        // day17,
        // day18,
        // day19,
        // day20,
        // day21,
        // day22,
        // day23,
        // day24,
        // day25,
    ]
}



// fn main() {
//     let 


//     let file_path = "../../input/2021/day1/input.txt";
//     let input_data = driver::driver::parse_input::<year2021::day1::Input>(&year2021::day1::parse_input, file_path);

//     year2021::day1::part_one(&input_data);
//     year2021::day1::part_two(&input_data);


// }
