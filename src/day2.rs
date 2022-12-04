use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
enum HandType {
    Rock,
    Paper,
    Scissors,
}

struct Strike {
    pub points: u16,
    pub hand_type: HandType,
}

impl Strike {
    pub fn from_letter(letter: &str) -> Result<Strike, &str> {
        match letter {
            "A" | "X" => Ok(Strike {
                points: 1,
                hand_type: HandType::Rock,
            }),
            "B" | "Y" => Ok(Strike {
                points: 2,
                hand_type: HandType::Paper,
            }),
            "C" | "Z" => Ok(Strike {
                points: 3,
                hand_type: HandType::Scissors,
            }),
            _ => Err("Invalid hand label"),
        }
    }
}

pub fn calculate_total_score(input: &str, valid_assumption: bool) -> Result<u16, &str> {
    let rounds = decode(input, valid_assumption)?;
    let score: u16 = rounds
        .iter()
        .map(|round| calculate_round_score(round).unwrap())
        .sum();

    return Ok(score);
}

fn calculate_round_score(round: &(Strike, Strike)) -> Result<u16, &str> {
    let mut matrix: HashMap<HandType, HashMap<HandType, u16>> = HashMap::new();
    let mut rock: HashMap<HandType, u16> = HashMap::new();
    let mut paper: HashMap<HandType, u16> = HashMap::new();
    let mut scissors: HashMap<HandType, u16> = HashMap::new();
    rock.insert(HandType::Rock, 3);
    rock.insert(HandType::Paper, 6);
    rock.insert(HandType::Scissors, 0);
    paper.insert(HandType::Rock, 0);
    paper.insert(HandType::Paper, 3);
    paper.insert(HandType::Scissors, 6);
    scissors.insert(HandType::Rock, 6);
    scissors.insert(HandType::Paper, 0);
    scissors.insert(HandType::Scissors, 3);
    matrix.insert(HandType::Rock, rock);
    matrix.insert(HandType::Paper, paper);
    matrix.insert(HandType::Scissors, scissors);

    if let Some(row) = matrix.get(&round.0.hand_type) {
        if let Some(score) = row.get(&round.1.hand_type) {
            return Ok(*score + round.1.points);
        }
    }

    return Err("HandType not found in matrix!");
}

fn decode(input: &str, valid_assumption: bool) -> Result<Vec<(Strike, Strike)>, &str> {
    let rows: Vec<&str> = input.trim().split('\n').collect();
    return rows
        .iter()
        .map(|row| {
            let output: Vec<&str> = row.trim().split(" ").collect();
            if output.len() != 2 {
                return Err("Invalid row data");
            }
            if valid_assumption {
                let enemy_strike = Strike::from_letter(output[0])?;
                let my_strike = match output[1] {
                    "X" => slave_of(&enemy_strike.hand_type),
                    "Y" => Strike::from_letter(output[0])?,
                    "Z" => master_of(&enemy_strike.hand_type),
                    _ => Strike::from_letter(output[0])?,
                };

                return Ok((enemy_strike, my_strike));
            } else {
                return Ok((
                    Strike::from_letter(output[0])?,
                    Strike::from_letter(output[1])?,
                ));
            }
        })
        .collect();
}

fn master_of(hand_type: &HandType) -> Strike {
    match hand_type {
        HandType::Rock => Strike {
            hand_type: HandType::Paper,
            points: 2,
        },
        HandType::Paper => Strike {
            hand_type: HandType::Scissors,
            points: 3,
        },
        HandType::Scissors => Strike {
            hand_type: HandType::Rock,
            points: 1,
        },
    }
}

fn slave_of(hand_type: &HandType) -> Strike {
    match hand_type {
        HandType::Rock => Strike {
            hand_type: HandType::Scissors,
            points: 3,
        },
        HandType::Paper => Strike {
            hand_type: HandType::Rock,
            points: 1,
        },
        HandType::Scissors => Strike {
            hand_type: HandType::Paper,
            points: 2,
        },
    }
}
