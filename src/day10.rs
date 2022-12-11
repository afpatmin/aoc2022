#[derive(Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

struct Processor {
    x: i32,
    cycle: i32,
    first_signal_cycle: i32,
    signal_cycle_step: i32,
    signal_strengths: Vec<i32>,
}

impl Processor {
    pub fn run_operation(&mut self, op: &Operation, render: bool) {
        let (cycles, dx) = match *op {
            Operation::Addx(n) => (2, n),
            Operation::Noop => (1, 0),
        };

        for _ in 0..cycles {
            if render {
                let row = (self.cycle as f32 / self.signal_cycle_step as f32).floor() as i32;
                let start = row * self.signal_cycle_step;
                let diff = (self.x + start) - self.cycle;
                let pixel = match diff {
                    -1..=1 => "#",
                    _ => ".",
                };
                print!("{}", pixel);
            }
            self.cycle = self.cycle + 1;

            if (self.cycle - self.first_signal_cycle) % (self.signal_cycle_step) == 0 {
                self.signal_strengths.push(self.cycle * self.x);
                if render {
                    println!("");
                }
            }
        }
        self.x = self.x + dx;
    }
}

pub fn sum_signal_strengths(input: &str, first_signal_cycle: i32, signal_cycle_step: i32) -> i32 {
    let operations = parse_input(input);
    let mut processor = Processor {
        x: 1,
        cycle: 0,
        first_signal_cycle,
        signal_cycle_step,
        signal_strengths: vec![],
    };

    for op in operations.iter() {
        processor.run_operation(op, false);
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

pub fn render_frame(input: &str) -> usize {
    let mut processor = Processor {
        x: 1,
        cycle: 0,
        first_signal_cycle: 0,
        signal_cycle_step: 40,
        signal_strengths: vec![],
    };

    let operations = parse_input(input);
    for op in operations.iter() {
        processor.run_operation(op, true);
    }
    0
}
