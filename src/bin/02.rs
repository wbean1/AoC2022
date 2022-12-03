pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score: u32 = 0;
    let lines = input.split("\n");
    for l in lines {
        let plays: Vec<&str> = l.split_whitespace().collect();
        total_score = total_score + score(plays.first().unwrap(), plays.last().unwrap()).unwrap();
    }
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score: u32 = 0;
    let lines = input.split("\n");
    for l in lines {
        let plays: Vec<&str> = l.split_whitespace().collect();
        total_score = total_score + score_part2(plays.first().unwrap(), plays.last().unwrap()).unwrap();
    }
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn score(theirs: &str, mine: &str) -> Option<u32> {
    let score: Option<u32> = match theirs {
        "A" => match mine {
            "X" => Some(1 + 3),
            "Y" => Some(2 + 6),
            "Z" => Some(3 + 0),
            _ => None,
        }
        "B" => match mine {
            "X" => Some(1 + 0),
            "Y" => Some(2 + 3),
            "Z" => Some(3 + 6),
            _ => None,
        }
        "C" => match mine {
            "X" => Some(1 + 6),
            "Y" => Some(2 + 0),
            "Z" => Some(3 + 3),
            _ => None,
        }
        _ => None,
    };
    score
}

fn score_part2(theirs: &str, result: &str) -> Option<u32> {
    let score: Option<u32> = match theirs {
        "A" => match result {
            "X" => Some(0 + 3),
            "Y" => Some(3 + 1),
            "Z" => Some(6 + 2),
            _ => None,
        }
        "B" => match result {
            "X" => Some(0 + 1),
            "Y" => Some(3 + 2),
            "Z" => Some(6 + 3),
            _ => None,
        }
        "C" => match result {
            "X" => Some(0 + 2),
            "Y" => Some(3 + 3),
            "Z" => Some(6 + 1),
            _ => None,
        }
        _ => None,
    };
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
