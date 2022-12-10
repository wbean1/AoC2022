pub fn part_one(input: &str) -> Option<u32> {
    let mut important_signal_sum: i32 = 0;
    let mut current_total: i32 = 1;
    let important_signal_cycles: [u32; 6] = [20, 60, 100, 140, 180, 220];
    let mut current_cycle: u32 = 0;
    for line in input.lines() {
        current_cycle = current_cycle + 1;
        if important_signal_cycles.contains(&current_cycle) {
            println!(
                "important signal on cycle: {}, line: {}, current_total: {}",
                current_cycle, line, current_total
            );
            important_signal_sum = important_signal_sum + (current_total * current_cycle as i32)
        }
        if line == "noop" {
            continue;
        }
        current_cycle = current_cycle + 1;
        if important_signal_cycles.contains(&current_cycle) {
            println!(
                "important signal on cycle: {}, line: {}, current_total: {}",
                current_cycle, line, current_total
            );
            important_signal_sum = important_signal_sum + (current_total * current_cycle as i32)
        }
        let num: i32 = line.split_once(" ").unwrap().1.parse().unwrap();

        current_total = current_total + num;
    }
    Some(important_signal_sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut important_signal_sum: i32 = 0;
    let mut current_total: i32 = 1;
    let important_signal_cycles: [u32; 6] = [40, 80, 120, 160, 200, 240];
    let mut str_arr = Vec::new();
    let mut current_cycle: u32 = 0;
    let mut current_str = String::from("");
    for line in input.lines() {
        if current_total >= ((current_cycle % 40) as i32 - 1)
            && current_total <= ((current_cycle % 40) as i32 + 1)
        {
            current_str.push('#');
        } else {
            current_str.push('.');
        }
        current_cycle = current_cycle + 1;
        if important_signal_cycles.contains(&current_cycle) {
            println!(
                "important signal on cycle: {}, line: {}, current_total: {}",
                current_cycle, line, current_total
            );
            important_signal_sum = important_signal_sum + (current_total * current_cycle as i32);
            str_arr.push(current_str);
            current_str = String::from("");
        }
        if line == "noop" {
            continue;
        }
        if current_total >= ((current_cycle % 40) as i32 - 1)
            && current_total <= ((current_cycle % 40) as i32 + 1)
        {
            current_str.push('#');
        } else {
            current_str.push('.');
        }
        current_cycle = current_cycle + 1;
        if important_signal_cycles.contains(&current_cycle) {
            println!(
                "important signal on cycle: {}, line: {}, current_total: {}",
                current_cycle, line, current_total
            );
            important_signal_sum = important_signal_sum + (current_total * current_cycle as i32);
            str_arr.push(current_str);
            current_str = String::from("");
        }
        let num: i32 = line.split_once(" ").unwrap().1.parse().unwrap();

        current_total = current_total + num;
    }
    for i in str_arr {
        println!("{}", i)
    }
    Some(important_signal_sum.try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(18000));
    }
}
