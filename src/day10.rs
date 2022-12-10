#[derive(Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

struct Processor {
    x: i32,
    cycle: i32,
    first_cycle: i32,
    cycle_step: i32,
    signal_strengths: Vec<i32>,
}

impl Processor {
    pub fn run_operation(&mut self, cycles: i32, x: i32) {
        for _ in 0..cycles {
            self.cycle = self.cycle + 1;
            if (self.cycle - self.first_cycle) % (self.cycle_step) == 0 {
                self.signal_strengths.push(self.cycle * self.x);
                println!("Cycle: {}", self.cycle);
            }
        }
        self.x = self.x + x;
    }
}

pub fn sum_signal_strengths(input: &str, first_cycle: i32, cycle_step: i32) -> i32 {
    let operations = parse_input(input);
    let mut processor = Processor {
        x: 1,
        cycle: 0,
        first_cycle,
        cycle_step,
        signal_strengths: vec![],
    };

    for op in operations.iter() {
        match op {
            Operation::Addx(n) => {
                processor.run_operation(2, *n);
            }
            Operation::Noop => {
                processor.run_operation(1, 0);
            }
        }
    }
    processor.signal_strengths.iter().fold(0, |acc, x| acc + x)
}

fn parse_input(input: &str) -> Vec<Operation> {
    input
        .split("\r\n")
        .map(|row| match row {
            "noop" => Operation::Noop,
            v => {
                let n: i32 = v
                    .split(" ")
                    .skip(1)
                    .next()
                    .expect("Should have been able to get two values")
                    .parse()
                    .expect("Should have been able to parse as i16");
                Operation::Addx(n)
            }
        })
        .collect()
}

pub fn b(input: &str) -> usize {
    0
}
