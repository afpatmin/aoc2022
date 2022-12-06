struct StockPile {
    crates: Vec<Vec<char>>,
}

impl StockPile {
    pub fn parse(mut input: Vec<&str>) -> Result<StockPile, String> {
        if let Some(indices) = input.last() {
            let mut crates: Vec<Vec<char>> = indices.split("  ").map(|_| Vec::new()).collect();
            input.pop();
            input.reverse();

            for row in input {
                for c in 0..(row.len() + 1) / 4 {
                    let mut col: String = row.chars().skip(4 * c).take(4).collect();
                    col = String::from(col.trim());

                    if !col.is_empty() {
                        crates[c].push(col.chars().nth(1).expect("Invalid col data"));
                    }
                }
            }
            return Ok(StockPile { crates });
        }

        Err(String::from("Invalid StockPile data"))
    }

    fn move_crates(&mut self, from_col: usize, to_col: usize, num_crates: u32) {
        let mut moved_crates: Vec<char> = vec![];
        for _ in 0..num_crates {
            if let Some(moved_crate) = self.crates[from_col].pop() {
                moved_crates.insert(0, moved_crate);
            }
        }

        for moved_crate in moved_crates {
            self.crates[to_col].push(moved_crate);
        }
    }

    pub fn perform_action(
        &mut self,
        instruction: &CraneInstruction,
        multiple_crates_per_lift: bool,
    ) {
        match instruction.action {
            CraneAction::Move(count) => {
                if multiple_crates_per_lift {
                    self.move_crates(instruction.from, instruction.to, count);
                } else {
                    for _ in 0..count {
                        self.move_crates(instruction.from, instruction.to, 1);
                    }
                }
            }
        }
    }

    pub fn report_top_crates(&self) -> String {
        let mut output = String::new();

        for col in &self.crates {
            output.push(*col.last().unwrap_or(&'-'));
        }

        output
    }
}

enum CraneAction {
    Move(u32),
}

struct CraneInstruction {
    action: CraneAction,
    from: usize,
    to: usize,
}

impl CraneInstruction {
    pub fn parse(input: &str) -> Result<CraneInstruction, String> {
        let parts: Vec<&str> = input.split(" ").collect();
        if parts.len() == 6 {
            if let Ok(num_crates) = parts[1].trim().parse::<u32>() {
                let action = match parts[0].trim() {
                    "move" => CraneAction::Move(num_crates),
                    _ => CraneAction::Move(num_crates),
                };
                if let Ok(from) = parts[3].trim().parse::<usize>() {
                    if let Ok(to) = parts[5].trim().parse::<usize>() {
                        return Ok(CraneInstruction {
                            action,
                            from: from - 1,
                            to: to - 1,
                        });
                    }
                }
            }
        }
        Err(format!("Invalid input format: {}", input))
    }
}

pub fn parse_instructions(input: &str, multiple_crates_per_lift: bool) -> String {
    let (mut stockpile, instructions) = decode(input);

    for instr in instructions {
        stockpile.perform_action(&instr, multiple_crates_per_lift);
    }
    stockpile.report_top_crates()
}

fn decode(input: &str) -> (StockPile, Vec<CraneInstruction>) {
    let data = String::from(input).replace("\n", "");
    let mut rows = data.split('\r');

    let stockpile_data: Vec<&str> = rows.by_ref().take_while(|r| *r != "").collect();
    let instructions_data: Vec<CraneInstruction> = rows
        .map(|row| CraneInstruction::parse(row).expect("Invalid instruction data"))
        .collect();

    (
        StockPile::parse(stockpile_data).expect("Invalid stockpile data"),
        instructions_data,
    )
}
