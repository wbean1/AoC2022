/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn split_in_half(input: &str) -> (&str, &str) {
    input.split_at(input.len() / 2)
}

pub fn find_common_char(input: &Vec<&str>) -> Option<char> {
    'outer: for c in input[0].chars() {
        for i in input[1..].iter() {
            if !i.contains(c) {
                continue 'outer;
            }
        }
        return Some(c);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_in_half_even() {
        let input: &str = "123456";
        assert_eq!(split_in_half(&input), ("123", "456"));
    }

    #[test]
    fn test_split_in_half_odd() {
        let input: &str = "1234567";
        assert_eq!(split_in_half(&input), ("123", "4567"));
    }

    #[test]
    fn test_find_common_char() {
        let input: Vec<&str> = ["abcdefGhij", "xyzG123"].to_vec();
        let expected: Option<char> = Some('G');
        assert_eq!(find_common_char(&input), expected);

        let input: Vec<&str> = ["abcdefGhijZ", "xyzG123Z", "Z"].to_vec();
        let expected: Option<char> = Some('Z');
        assert_eq!(find_common_char(&input), expected);
    }
}
