struct Range {
    min: u32,
    max: u32,
}

fn line_to_ranges(line: &str) -> (Range, Range) {
    let (elf1, elf2) = line.split_once(",").unwrap();
    let (e1_min, e1_max) = elf1.split_once("-").unwrap();
    let (e2_min, e2_max) = elf2.split_once("-").unwrap();
    return (
        Range {
            min: e1_min.parse().unwrap(),
            max: e1_max.parse().unwrap(),
        },
        Range {
            min: e2_min.parse().unwrap(),
            max: e2_max.parse().unwrap(),
        },
    );
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    for line in input.lines() {
        let (elf1, elf2) = line_to_ranges(line);
        if (elf1.min <= elf2.min && elf1.max >= elf2.max)
            || (elf2.min <= elf1.min && elf2.max >= elf1.max)
        {
            count = count + 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    for line in input.lines() {
        let (elf1, elf2) = line_to_ranges(line);
        if !((elf1.max < elf2.min) || (elf2.max < elf1.min)) {
            count = count + 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
