struct IdRange {
    min: u16,
    max: u16,
}

impl IdRange {
    pub fn from_string(data: &str) -> IdRange {
        let mut parts = data.split('-');

        if let Some(str_min) = parts.next() {
            if let Some(str_max) = parts.next() {
                let min = match str_min.trim().parse() {
                    Ok(min) => min,
                    _ => 0,
                };
                let max = match str_max.trim().parse() {
                    Ok(max) => max,
                    _ => 0,
                };
                return IdRange { min: min, max: max };
            }
        }
        IdRange { min: 0, max: 0 }
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
