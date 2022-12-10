use std::collections::HashMap;

struct Position {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut positions = HashMap::new();
    for line in input.lines() {
        let (direction, num_str) = line.split_once(" ").unwrap();
        let num: u32 = num_str.parse().unwrap();
        for _ in 0..num {
            (head, tail) = move_rope(direction, head, tail);
            let position_str = format!("{},{}", tail.x, tail.y);
            positions.insert(position_str, true);
        }
    }
    u32::try_from(positions.len()).ok()
}

fn move_rope(direction: &str, head: Position, tail: Position) -> (Position, Position) {
    let new_head: Position;
    let mut new_tail: Position;
    match direction {
        "R" => {
            // increase x;  same y
            new_head = Position {
                x: head.x + 1,
                y: head.y,
            };
            new_tail = Position {
                x: tail.x,
                y: tail.y,
            };
            if (new_head.x - tail.x).abs() > 1 {
                new_tail.x = new_tail.x + 1;
                new_tail.y = new_head.y;
            }
            (new_head, new_tail)
        }
        "L" => {
            // decrease x; same y
            new_head = Position {
                x: head.x - 1,
                y: head.y,
            };
            new_tail = Position {
                x: tail.x,
                y: tail.y,
            };
            if (new_head.x - tail.x).abs() > 1 {
                new_tail.x = new_tail.x - 1;
                new_tail.y = new_head.y;
            }
            (new_head, new_tail)
        }
        "U" => {
            // increase y; same x
            new_head = Position {
                x: head.x,
                y: head.y + 1,
            };
            new_tail = Position {
                x: tail.x,
                y: tail.y,
            };
            if (new_head.y - tail.y).abs() > 1 {
                new_tail.y = new_tail.y + 1;
                new_tail.x = new_head.x;
            }
            (new_head, new_tail)
        }
        "D" => {
            // decrease y; same x
            new_head = Position {
                x: head.x,
                y: head.y - 1,
            };
            new_tail = Position {
                x: tail.x,
                y: tail.y,
            };
            if (new_head.y - tail.y).abs() > 1 {
                new_tail.y = new_tail.y - 1;
                new_tail.x = new_head.x;
            }
            (new_head, new_tail)
        }
        _ => panic!("UNKNOWN DIRECTION {}", direction),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_larger() {
        let input = advent_of_code::read_file("examples", 99);
        assert_eq!(part_two(&input), Some(36));
    }
}
