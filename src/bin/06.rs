use advent_of_code::helpers::contains_dupe_char;
use substring::Substring;

pub fn part_one(input: &str) -> Option<u32> {
    let char_vec: Vec<char> = input.chars().collect();
    for i in 3..char_vec.len() {
        if (char_vec[i] != char_vec[i - 1])
            && (char_vec[i] != char_vec[i - 2])
            && (char_vec[i] != char_vec[i - 3])
            && (char_vec[i - 1] != char_vec[i - 2])
            && (char_vec[i - 1] != char_vec[i - 3])
            && (char_vec[i - 2] != char_vec[i - 3])
        {
            return u32::try_from(i + 1).ok();
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let str = input.to_string();
    for i in 14..str.len() {
        let s = str.substring(i - 14, i);
        if !contains_dupe_char(s) {
            return u32::try_from(i).ok();
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_part_one() {
        let mut tests = HashMap::new();
        tests.insert("bvwbjplbgvbhsrlpgdmjqwftvnc", 5);
        tests.insert("nppdvjthqldpwncqszvftbrmjlhg", 6);
        tests.insert("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        tests.insert("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);
        for (&k, &v) in tests.iter() {
            assert_eq!(part_one(k), u32::try_from(v).ok());
        }
    }

    #[test]
    fn test_part_two() {
        let mut tests = HashMap::new();
        tests.insert("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19);
        tests.insert("bvwbjplbgvbhsrlpgdmjqwftvncz", 23);
        tests.insert("nppdvjthqldpwncqszvftbrmjlhg", 23);
        tests.insert("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29);
        tests.insert("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26);
        for (&k, &v) in tests.iter() {
            assert_eq!(part_two(k), u32::try_from(v).ok());
        }
    }
}
