use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
    // for the test to pass...
    // let mut stacks: Vec<Vec<&str>> = vec![
    //     vec!["Z", "N"],
    //     vec!["M", "C", "D"],
    //     vec!["P"],
    // ];
    //
    // for the actual run...
    let mut stacks: Vec<Vec<&str>> = vec![
        vec!["T","P","Z","C","S","L","Q","N"],
        vec!["L","P","T","V","H","C","G"],
        vec!["D","C","Z","F"],
        vec!["G","W","T","D","L","M","V","C"],
        vec!["P","W","C"],
        vec!["P","F","J","D","C","T","S","Z"],
        vec!["V","W","G","B","D"],
        vec!["N","J","S","Q","H","W"],
        vec!["R","C","Q","F","S","L","V"],
    ];
    for line in input.lines() {
        let (quantity, from_col, to_col) = parse_line(line);
        for _i in 0..quantity {
            let temp = stacks[(from_col - 1) as usize].pop().unwrap();
            stacks[(to_col - 1) as usize].push(temp);
        }
    }
    
    Some(top_of_stacks(stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    // for the test to pass...
    // let mut stacks: Vec<Vec<&str>> = vec![
    //     vec!["Z", "N"],
    //     vec!["M", "C", "D"],
    //     vec!["P"],
    // ];
    
    // for the actual run...
    let mut stacks: Vec<Vec<&str>> = vec![
        vec!["T","P","Z","C","S","L","Q","N"],
        vec!["L","P","T","V","H","C","G"],
        vec!["D","C","Z","F"],
        vec!["G","W","T","D","L","M","V","C"],
        vec!["P","W","C"],
        vec!["P","F","J","D","C","T","S","Z"],
        vec!["V","W","G","B","D"],
        vec!["N","J","S","Q","H","W"],
        vec!["R","C","Q","F","S","L","V"],
    ];
    for line in input.lines() {
        let (quantity, from_col, to_col) = parse_line(line);
        let mut spool: Vec<&str> = Vec::new();
        for _i in 0..quantity {
            let temp = stacks[(from_col - 1) as usize].pop().unwrap();
            spool.push(temp)
        }
        for _i in 0..quantity {
            stacks[(to_col - 1) as usize].push(spool.pop().unwrap())
        }
    }
    
    Some(top_of_stacks(stacks))
}

fn top_of_stacks(mut stacks: Vec<Vec<&str>>) -> String {
    let mut answer = String::from("");
    for i in 0..stacks.len() {
        answer.push_str(stacks[i].pop().unwrap());
    }
    answer
}

fn parse_line(line: &str) -> (u32, u32, u32) {
    let re = Regex::new(r"move (?P<one>\d+) from (?P<two>\d+) to (?P<three>\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let (one, two, three): (u32,u32,u32);
    one = caps.name("one").unwrap().as_str().parse().unwrap();
    two = caps.name("two").unwrap().as_str().parse().unwrap();
    three = caps.name("three").unwrap().as_str().parse().unwrap();
    return (one, two, three);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
