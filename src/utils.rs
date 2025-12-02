pub fn input_lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .into_iter()
        .filter_map(|line| match line.trim() {
            "" => None,
            x => Some(x),
        })
}
