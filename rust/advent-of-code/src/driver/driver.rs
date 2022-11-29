use std::fs;

pub fn parse_input<T>(parse: &dyn Fn(String) -> T, file_path: &str) -> T {
  let contents = fs::read_to_string(file_path).unwrap();
  parse(contents)
}
