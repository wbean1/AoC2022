use trees::{tr,Tree,Node};

enum DataType {
    File,
    Dir,
}

struct Data {
    name: String,
    datatype: DataType,
    size: Option<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut tree: Tree<Data> = Tree::new(Data {name: "/".to_string(), datatype: DataType::Dir, size: None});
    
    let iter = tree.iter();
    // create our tree by reading in commands & output
    for line in input.lines().skip(1) {

    }

    // traverse the tree recursively filling in size

    // traverse the tree again summing dirs <= 100,000

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let input = advent_of_code::read_file("examples", 7);
    //     assert_eq!(part_one(&input), Some(95437));
    // }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 7);
    //     assert_eq!(part_two(&input), None);
    // }
}
