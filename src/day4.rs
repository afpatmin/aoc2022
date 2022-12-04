use std::num::ParseIntError;

struct IdRange {
    min: u16,
    max: u16,
}

impl IdRange {
    pub fn from_string(data: &str) -> IdRange {
        let mut parts = data.split('-');

        IdRange {
            min: parts
                .next()
                .expect("Bad IdRange input")
                .parse()
                .expect("Not a valid integer"),
            max: parts
                .next()
                .expect("Bad IdRange input")
                .parse()
                .expect("Not a valid integer"),
        }
    }

    pub fn is_fully_contained_within(&self, other: &IdRange) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    pub fn overlaps(&self, other: &IdRange) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

pub fn count_fully_overlapping(input: &str) -> u16 {
    let ranges = decode(&input);
    let mut overlap_count: u16 = 0;
    for range in ranges {
        if range.0.is_fully_contained_within(&range.1)
            || range.1.is_fully_contained_within(&range.0)
        {
            overlap_count += 1;
        }
    }
    overlap_count
}

pub fn count_overlapping(input: &str) -> u16 {
    let ranges = decode(&input);
    let mut overlap_count: u16 = 0;
    for range in ranges {
        if range.0.overlaps(&range.1) || range.1.overlaps(&range.0) {
            overlap_count += 1;
        }
    }
    overlap_count
}

fn decode(input: &str) -> Vec<(IdRange, IdRange)> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            let mut parts = row.split(',');
            if let Some(first) = parts.next() {
                if let Some(second) = parts.next() {
                    return (IdRange::from_string(first), IdRange::from_string(second));
                }
            }
            panic!("Bad row input");
        })
        .collect()
}
