/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn split_in_half(input: &str) -> (&str, &str) {
   input.split_at(input.len()/2)
}

pub fn find_common_char(in1: &str, in2: &str) -> Option<char> {
    for c in in1.chars() {
        if in2.contains(c) {
            return Some(c)
        }
    }
    None
}

pub fn find_common_char_three(in1: &str, in2: &str, in3: &str) -> Option<char> {
    for c in in1.chars() {
        if in2.contains(c) && in3.contains(c) {
            return Some(c)
        }
    }
    None
}

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_in_half_even() {
        let input: &str = "123456";
        assert_eq!(split_in_half(&input), ("123","456"));
    }

    #[test]
    fn test_split_in_half_odd() {
        let input: &str = "1234567";
        assert_eq!(split_in_half(&input), ("123","4567"));
    }

    #[test]
    fn test_find_common_char() {
        let (in1, in2):(&str,&str) = ("abcdefGhij", "xyzG123");
        let expected: Option<char> = Some('G');
        assert_eq!(find_common_char(in1, in2), expected);
    }

}