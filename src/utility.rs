pub fn split_lines(s: &str) -> Vec<String> {
    s.lines().map(|s| s.to_owned()).collect::<Vec<String>>()
}
