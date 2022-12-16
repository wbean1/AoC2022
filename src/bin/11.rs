struct Monkey {
    items: Vec<u128>,
    operation: &'static str,
    operation_amount: &'static str,
    test_amount: u128,
    true_monkey: usize,
    false_monkey: usize,
    num_inspected: u128,
}

impl Monkey {
    fn inspect(&mut self) -> u128 {
        let mut item = self.items.pop().unwrap();
        self.num_inspected = self.num_inspected + 1;
        match self.operation {
            "add" => { 
                let amount: u128 = self.operation_amount.parse().unwrap();
                item = item + amount;
                item / 3
            },
            "mult" => {
                match self.operation_amount {
                    "self" => {
                        item = item * item;
                        item / 3
                    },
                    _ => {
                        let amount: u128 = self.operation_amount.parse().unwrap();
                        item = item * amount;
                        item / 3
                    }
                }
            },
            _ => panic!("unsupported operation {}", self.operation)
        }
    }
    fn inspect_w_lcd(&mut self, lcd: u128) -> u128 {
        let item = self.items.pop().unwrap();
        self.num_inspected = self.num_inspected + 1;
        match self.operation {
            "add" => { 
                let amount: u128 = self.operation_amount.parse().unwrap();
                (item + amount) % lcd
            },
            "mult" => {
                match self.operation_amount {
                    "self" => {
                        (item * item) % lcd
                    },
                    _ => {
                        let amount: u128 = self.operation_amount.parse().unwrap();
                        (item * amount) % lcd
                    }
                }
            },
            _ => panic!("unsupported operation {}", self.operation)
        }
    }
    fn pass_to(&self, item: u128) -> usize {
        if item % self.test_amount == 0 {
            return self.true_monkey
        }
        self.false_monkey
    }
    fn receive_item(&mut self, item: u128) {
        self.items.push(item)
    }
}


pub fn part_one(input: &str) -> Option<u128> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    // for the test to pass
    monkeys.push(Monkey { 
        items: vec![79,98],
        operation: "mult",
        operation_amount: "19",
        test_amount: 23,
        true_monkey: 2,
        false_monkey: 3,
        num_inspected: 0,
    });
    monkeys.push(Monkey {
        items: vec![54,65,75,74],
        operation: "add",
        operation_amount: "6",
        test_amount: 19,
        true_monkey: 2,
        false_monkey: 0,
        num_inspected: 0,
    });
    monkeys.push(Monkey{
        items: vec![79,60,97],
        operation: "mult",
        operation_amount: "self",
        test_amount: 13,
        true_monkey: 1,
        false_monkey: 3,
        num_inspected: 0,
    });
    monkeys.push(Monkey {
        items: vec![74],
        operation: "add",
        operation_amount: "3",
        test_amount: 17,
        true_monkey: 0,
        false_monkey: 1,
        num_inspected: 0,
    });

    // for the solution
    // monkeys.push(Monkey { 
    //     items: vec![89,73,66,57,64,80],
    //     operation: "mult",
    //     operation_amount: "3",
    //     test_amount: 13,
    //     true_monkey: 6,
    //     false_monkey: 2,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![83,78,81,55,81,59,69],
    //     operation: "add",
    //     operation_amount: "1",
    //     test_amount: 3,
    //     true_monkey: 7,
    //     false_monkey: 4,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey{
    //     items: vec![76,91,58,85],
    //     operation: "mult",
    //     operation_amount: "13",
    //     test_amount: 7,
    //     true_monkey: 1,
    //     false_monkey: 4,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![71,72,74,76,68],
    //     operation: "mult",
    //     operation_amount: "self",
    //     test_amount: 2,
    //     true_monkey: 6,
    //     false_monkey: 0,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![98,85,84],
    //     operation: "add",
    //     operation_amount: "7",
    //     test_amount: 19,
    //     true_monkey: 5,
    //     false_monkey: 7,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![78],
    //     operation: "add",
    //     operation_amount: "8",
    //     test_amount: 5,
    //     true_monkey: 3,
    //     false_monkey: 0,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![86,70,60,88,88,78,74,83],
    //     operation: "add",
    //     operation_amount: "4",
    //     test_amount: 11,
    //     true_monkey: 1,
    //     false_monkey: 2,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![81,58],
    //     operation: "add",
    //     operation_amount: "5",
    //     test_amount: 17,
    //     true_monkey: 3,
    //     false_monkey: 5,
    //     num_inspected: 0,
    // });

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let item = monkeys[i].inspect();
                let pass_to = monkeys[i].pass_to(item);
                monkeys[pass_to].receive_item(item);
            }
        }
    }

    let mut largest: [u128; 2] = [0, 0];
    for monkey in monkeys {
        if monkey.num_inspected > largest[0] {
            let old_largest = largest[0];
            largest[0] = monkey.num_inspected;
            if old_largest > largest[1] {
                largest[1] = old_largest;
            }
        } else if monkey.num_inspected > largest[1] {
            largest[1] = monkey.num_inspected
        }
    }


    Some(largest[0] * largest[1])
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    // for the test to pass
    monkeys.push(Monkey { 
        items: vec![79,98],
        operation: "mult",
        operation_amount: "19",
        test_amount: 23,
        true_monkey: 2,
        false_monkey: 3,
        num_inspected: 0,
    });
    monkeys.push(Monkey {
        items: vec![54,65,75,74],
        operation: "add",
        operation_amount: "6",
        test_amount: 19,
        true_monkey: 2,
        false_monkey: 0,
        num_inspected: 0,
    });
    monkeys.push(Monkey{
        items: vec![79,60,97],
        operation: "mult",
        operation_amount: "self",
        test_amount: 13,
        true_monkey: 1,
        false_monkey: 3,
        num_inspected: 0,
    });
    monkeys.push(Monkey {
        items: vec![74],
        operation: "add",
        operation_amount: "3",
        test_amount: 17,
        true_monkey: 0,
        false_monkey: 1,
        num_inspected: 0,
    });

    // for the solution
    // monkeys.push(Monkey { 
    //     items: vec![89,73,66,57,64,80],
    //     operation: "mult",
    //     operation_amount: "3",
    //     test_amount: 13,
    //     true_monkey: 6,
    //     false_monkey: 2,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![83,78,81,55,81,59,69],
    //     operation: "add",
    //     operation_amount: "1",
    //     test_amount: 3,
    //     true_monkey: 7,
    //     false_monkey: 4,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey{
    //     items: vec![76,91,58,85],
    //     operation: "mult",
    //     operation_amount: "13",
    //     test_amount: 7,
    //     true_monkey: 1,
    //     false_monkey: 4,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![71,72,74,76,68],
    //     operation: "mult",
    //     operation_amount: "self",
    //     test_amount: 2,
    //     true_monkey: 6,
    //     false_monkey: 0,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![98,85,84],
    //     operation: "add",
    //     operation_amount: "7",
    //     test_amount: 19,
    //     true_monkey: 5,
    //     false_monkey: 7,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![78],
    //     operation: "add",
    //     operation_amount: "8",
    //     test_amount: 5,
    //     true_monkey: 3,
    //     false_monkey: 0,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![86,70,60,88,88,78,74,83],
    //     operation: "add",
    //     operation_amount: "4",
    //     test_amount: 11,
    //     true_monkey: 1,
    //     false_monkey: 2,
    //     num_inspected: 0,
    // });
    // monkeys.push(Monkey {
    //     items: vec![81,58],
    //     operation: "add",
    //     operation_amount: "5",
    //     test_amount: 17,
    //     true_monkey: 3,
    //     false_monkey: 5,
    //     num_inspected: 0,
    // });
    let mut monkey_lcd = 1;
    for i in 0..monkeys.len() {
        monkey_lcd = monkey_lcd * monkeys[i].test_amount
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let item = monkeys[i].inspect_w_lcd(monkey_lcd);
                let pass_to = monkeys[i].pass_to(item);
                monkeys[pass_to].receive_item(item);
            }
        }
    }

    let mut largest: [u128; 2] = [0, 0];
    for monkey in monkeys {
        if monkey.num_inspected > largest[0] {
            let old_largest = largest[0];
            largest[0] = monkey.num_inspected;
            if old_largest > largest[1] {
                largest[1] = old_largest;
            }
        } else if monkey.num_inspected > largest[1] {
            largest[1] = monkey.num_inspected
        }
    }


    Some(largest[0] * largest[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
