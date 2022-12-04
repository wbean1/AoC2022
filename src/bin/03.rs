use advent_of_code::helpers::{find_common_char, split_in_half};

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut scores: Vec<u32> = Vec::new();
    for l in lines {
        let (part1, part2) = split_in_half(l);
        let common_char: char = find_common_char([part1, part2].to_vec().as_ref()).unwrap();
        scores.push(score(common_char) + 1);
    }
    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut scores: Vec<u32> = Vec::new();
    let mut current_elfs: Vec<&str> = Vec::new();
    for (i, l) in lines.enumerate() {
        current_elfs.push(l);
        if i % 3 == 2 {
            let common_char: char = find_common_char(&current_elfs).unwrap();
            scores.push(score(common_char) + 1);
            current_elfs.clear();
        }
    }
    Some(scores.iter().sum())
}

fn score(c: char) -> u32 {
    let scores = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    scores.find(c).unwrap().try_into().unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
