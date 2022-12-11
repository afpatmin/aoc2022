use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Multiply(i16),
    Add(i16),
    Square,
}

impl Operation {
    // Implemented proper error handling here as a learning exercise (premature optimization...)
    pub fn parse(input: &str) -> Result<Operation, &str> {
        if let Some(part) = input.split("Operation: new = ").skip(1).next() {
            let mut it = part.split(" ").skip(1);
            match it.next() {
                Some("+") => {
                    if let Some(amount_str) = it.next() {
                        match amount_str.parse() {
                            Ok(amount) => Ok(Operation::Add(amount)),
                            _ => Err("Couldn't parse the value to add"),
                        }
                    } else {
                        Err("Couldn't parse add operation")
                    }
                }
                Some("*") => {
                    if let Some(amount_str) = it.next() {
                        if amount_str == "old" {
                            return Ok(Operation::Square);
                        } else {
                            match amount_str.parse() {
                                Ok(amount) => Ok(Operation::Multiply(amount)),
                                _ => Err("Could not parse the value to multiply with"),
                            }
                        }
                    } else {
                        Err("Couldn't parse multiply operation")
                    }
                }
                _ => Err("Could not parse operation symbol"),
            }
        } else {
            Err("Could not parse operation input")
        }
    }
}

#[derive(PartialEq, Eq)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: u32,
    on_test_success: u32,
    on_test_fail: u32,
    num_inspections: u32,
}
impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num_inspections.cmp(&other.num_inspections)
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num_inspections.partial_cmp(&other.num_inspections)
    }
}

impl Monkey {
    pub fn parse(input: &str) -> Result<Monkey, &str> {
        let mut rows = input.split("\r\n");
        let items: Vec<u32> = rows
            .by_ref()
            .skip(1)
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let operation = match rows.by_ref().next() {
            Some(op) => Operation::parse(op)?,
            None => return Err("Could not parse Monkey operation"),
        };

        let test = rows
            .by_ref()
            .next()
            .unwrap()
            .split("divisible by ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let on_test_success = rows
            .by_ref()
            .next()
            .unwrap()
            .split("If true: throw to monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let on_test_fail = rows
            .by_ref()
            .next()
            .unwrap()
            .split("If false: throw to monkey ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        Ok(Monkey {
            items,
            operation,
            test,
            on_test_success,
            on_test_fail,
            num_inspections: 0,
        })
    }

    /// Return a tuple where index 1 is item worry level and index 2 is the target monkey
    pub fn do_turn(&mut self) -> Option<(u32, u32)> {
        if self.items.is_empty() {
            return None;
        } else {
            let mut item = self.items.remove(0);
            item = match self.operation {
                Operation::Add(n) => item + n as u32,
                Operation::Multiply(n) => item * n as u32,
                Operation::Square => item * item,
            };
            item = (item as f32 / 3.0).floor() as u32;

            let target_monkey = match item % self.test == 0 {
                true => self.on_test_success,
                false => self.on_test_fail,
            };

            self.num_inspections = self.num_inspections + 1;

            return Some((item, target_monkey));
        }
    }

    pub fn add_item(&mut self, item: u32) {
        self.items.push(item);
    }
}

pub fn evaluate_monkey_business_level(input: &str, rounds: usize) -> usize {
    let monkeys = input.split("\r\n\r\n");
    let mut monkeys: Vec<Monkey> = monkeys.map(|m| Monkey::parse(m).unwrap()).collect();

    for round in 1..=rounds {
        for monkey in 0..monkeys.len() {
            while let Some((item, target)) = monkeys[monkey].do_turn() {
                monkeys[target as usize].add_item(item);
            }
        }
    }
    monkeys.sort();
    monkeys.reverse();
    (monkeys[0].num_inspections * monkeys[1].num_inspections) as usize
}
