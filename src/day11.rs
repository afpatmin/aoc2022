enum Operation {
    multiply(i32),
    add(i32),
    square,
}

struct Monkey {
    items: Vec<u16>,
    operation: Operation,
    divisible_by: u16,
}

impl Monkey {
    pub fn parse(input: &str) -> Monkey {
        let mut rows = input.split("\r\n");
        let str_items = rows
            .by_ref()
            .skip(1)
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap();
        let str_operation = rows
            .by_ref()
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap();
        let divisible_by = rows
            .by_ref()
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap();

        Monkey {
            items: vec![],
            operation: Operation::multiply(3),
            divisible_by: 4,
        }
    }
}
