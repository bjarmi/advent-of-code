
pub fn parse_grid(input: &[String]) -> Vec<Vec<bool>> {
    input
        .iter()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(|(x, c)| c == '@').collect())
        .collect()
}
