mod driver;
mod year2021;


fn main() {
    let file_path = "../../input/2021/day1/input.txt";
    let input_data = driver::driver::parse_input::<year2021::day1::Input>(&year2021::day1::parse_input, file_path);

    year2021::day1::part_one(&input_data);
    year2021::day1::part_two(&input_data);
}
