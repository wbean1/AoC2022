pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        matrix.push(line.as_bytes().to_vec())
    }

    let mut count: u32 = 0;
    for (i, l) in matrix.iter().enumerate() {
        for (j, _) in l.iter().enumerate() {
            // highest from left? (variable j, constant i)
            let mut visible_from_left: bool = true;
            for m in 0..j {
                if matrix[i][m] >= matrix[i][j] {
                    visible_from_left = false;
                }
            }
            if visible_from_left {
                count = count + 1;
                continue;
            }

            // highest from right? (variable j, constant i)
            let mut visible_from_right: bool = true;
            for n in j + 1..l.len() {
                if matrix[i][n] >= matrix[i][j] {
                    visible_from_right = false;
                }
            }
            if visible_from_right {
                count = count + 1;
                continue;
            }

            // higest from top? (variable i, constant j)
            let mut visible_from_top: bool = true;
            for n in 0..i {
                if matrix[n][j] >= matrix[i][j] {
                    visible_from_top = false;
                }
            }
            if visible_from_top {
                count = count + 1;
                continue;
            }

            // highest from bottom? (variable i, constant j)
            let mut visible_from_bottom: bool = true;
            for n in i + 1..matrix.len() {
                if matrix[n][j] >= matrix[i][j] {
                    visible_from_bottom = false;
                }
            }
            if visible_from_bottom {
                count = count + 1;
                continue;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        matrix.push(line.as_bytes().to_vec())
    }

    let mut max_score: u32 = 0;
    for (i, l) in matrix.iter().enumerate() {
        for (j, _) in l.iter().enumerate() {
            // how far can see left? (variable j, constant i)
            let mut view_to_left: u32 = 0;
            for m in 1..j + 1 {
                view_to_left = view_to_left + 1;
                if matrix[i][j - m] >= matrix[i][j] {
                    break;
                }
            }

            // how far can see right? (variable j, constant i)
            let mut view_to_right: u32 = 0;
            for n in j + 1..l.len() {
                view_to_right = view_to_right + 1;
                if matrix[i][n] >= matrix[i][j] {
                    break;
                }
            }

            // higest from top? (variable i, constant j)
            let mut view_to_top: u32 = 0;
            for n in 1..i + 1 {
                view_to_top = view_to_top + 1;
                if matrix[i - n][j] >= matrix[i][j] {
                    break;
                }
            }

            // highest from bottom? (variable i, constant j)
            let mut view_to_bottom: u32 = 0;
            for n in i + 1..matrix.len() {
                view_to_bottom = view_to_bottom + 1;
                if matrix[n][j] >= matrix[i][j] {
                    break;
                }
            }

            let score = view_to_left * view_to_right * view_to_top * view_to_bottom;
            if score > max_score {
                max_score = score
            }
        }
    }

    Some(max_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
