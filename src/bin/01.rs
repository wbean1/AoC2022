use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let mut max: u32 = 0;
    let mut current_sum: u32 = 0;
    let lines = input.split("\n");

    for l in lines {
        if l == "" {
            current_sum = 0
        } else {
            let n: u32 = FromStr::from_str(l).unwrap();
            current_sum = current_sum + n;
            if current_sum > max {
                max = current_sum;
            }
        }
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut elfs = Vec::new();
    let mut current_sum: u32 = 0;
    for l in lines {
        if l == "" {
            elfs.push(current_sum);
            current_sum = 0;
        } else {
            let n: u32 = FromStr::from_str(l).unwrap();
            current_sum = current_sum + n;
        }
    }
    elfs.push(current_sum);
    elfs.sort();
    Some(elfs.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let expected: Option<u32> = Some(24000);
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), expected);
    }

    #[test]
    fn test_part_two() {
        let expected: Option<u32> = Some(45000);
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), expected);
    }
}
