#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Multiply(i64),
    Add(i64),
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
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    on_test_success: u64,
    on_test_fail: u64,
    num_inspections: u64,
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
        let items: Vec<u64> = rows
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
            .parse::<u64>()
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
    pub fn do_turn(&mut self, relief_factor: u8, modulo: u64) -> Option<(u64, u64)> {
        if self.items.is_empty() {
            return None;
        } else {
            let mut worrylvl = self.items.remove(0);

            worrylvl = match self.operation {
                Operation::Add(n) => worrylvl + n as u64,
                Operation::Multiply(n) => worrylvl * n as u64,
                Operation::Square => worrylvl * worrylvl,
            };

            worrylvl = worrylvl / u64::from(relief_factor);
            worrylvl = worrylvl % modulo;

            let target_monkey = match worrylvl % self.test == 0 {
                true => self.on_test_success,
                false => self.on_test_fail,
            };

            self.num_inspections = self.num_inspections + 1;

            return Some((worrylvl, target_monkey));
        }
    }

    pub fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

pub fn evaluate_monkey_business_level(input: &str, rounds: usize, relief_factor: u8) -> usize {
    let mut modulo = 1;
    let monkeys = input.split("\r\n\r\n");
    let mut monkeys: Vec<Monkey> = monkeys.map(|m| Monkey::parse(m).unwrap()).collect();

    for m in &monkeys {
        modulo = modulo * m.test;
    }

    for _ in 1..=rounds {
        for monkey in 0..monkeys.len() {
            while let Some((item, target)) = monkeys[monkey].do_turn(relief_factor, modulo) {
                monkeys[target as usize].add_item(item);
            }
        }
    }
    monkeys.sort();

    (monkeys[monkeys.len() - 1].num_inspections * monkeys[monkeys.len() - 2].num_inspections)
        as usize
}
