pub fn parse_line(line: String) -> i32 {
    i32::from_str_radix(&line, 2).expect("unable to parse int from string")
}
