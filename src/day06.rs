fn parse_input(input: &str) -> Vec<Vec<&[u8]>> {
    input
        .trim()
        .split("\n\n")
        .map(|section| section.trim().lines().map(|s| s.as_bytes()).collect())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|grid| {
            let mut m: Vec<bool> = vec![false; 26];
            grid.iter().for_each(|row| {
                row.iter().for_each(|ch| m[(ch - b'a') as usize] = true);
            });
            m.iter().filter(|it| **it).count()
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|grid| {
            let n = grid.len();
            let mut m = vec![0; 26];
            grid.iter().for_each(|row| {
                row.iter().for_each(|ch| {
                    let i = (ch - b'a') as usize;
                    m[i] += 1;
                });
            });
            m.into_iter().filter(|it| *it == n).count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 11);
        assert_eq!(part_two(&input), 6);
    }
}
