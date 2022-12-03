use advent_of_code::helpers::{find_common_char, find_common_char_three, split_in_half};

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut scores: Vec<u32> = Vec::new();
    for l in lines {
        let (part1, part2) = split_in_half(l);
        let common_char: char = find_common_char(part1, part2).unwrap();
        scores.push(score(common_char) + 1);

    }
    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut scores: Vec<u32> = Vec::new();
    let mut current_elfs: Vec<&str> = Vec::new();
    for (i, l) in lines.enumerate() {
        if i % 3 == 2 {
            current_elfs.push(l);
            let common_char: char = find_common_char_three(current_elfs.pop().unwrap(), current_elfs.pop().unwrap(), current_elfs.pop().unwrap()).unwrap();
            scores.push(score(common_char) + 1);
        } else {
            current_elfs.push(l);
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
