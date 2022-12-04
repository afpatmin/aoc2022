use std::collections::{HashMap, HashSet};

struct Gnome {
    comp1: Vec<char>,
    comp2: Vec<char>,
}

impl Gnome {
    pub fn from_string(data: &str) -> Gnome {
        let parts = data.split_at(data.len() / 2);
        Gnome {
            comp1: parts.0.chars().collect(),
            comp2: parts.1.chars().collect(),
        }
    }

    pub fn contains(&self, item: &char) -> bool {
        self.comp1.contains(item) || self.comp2.contains(item)
    }

    pub fn find_doublets(&self) -> Vec<char> {
        let mut output: HashSet<char> = HashSet::new();
        for x in &self.comp1 {
            if self.comp2.contains(x) {
                output.insert(x.clone());
            }
        }
        return output.into_iter().collect();
    }
}

fn find_badge(gnomes: (Gnome, Gnome, Gnome)) -> Option<char> {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    for char in chars {
        if gnomes.0.contains(&char) && gnomes.1.contains(&char) && gnomes.2.contains(&char) {
            return Some(char);
        }
    }
    return None;
}

pub fn calc_badge_priorities(input: &String) -> u16 {
    let prios = get_prios();
    let mut rows = decode(input);
    let mut total = 0;
    while rows.len() >= 3 {
        if let Some(badge) = find_badge((
            rows.pop().unwrap(),
            rows.pop().unwrap(),
            rows.pop().unwrap(),
        )) {
            if let Some(prio) = prios.get(&badge) {
                total += prio;
            }
        }
    }

    total
}

fn decode(input: &String) -> Vec<Gnome> {
    input
        .trim()
        .split('\n')
        .map(|row| Gnome::from_string(row))
        .collect()
}

pub fn calc_doublet_priorities(input: &String) -> u16 {
    let prios = get_prios();
    decode(input)
        .iter()
        .map(|gnome| {
            let doubles = gnome.find_doublets();
            if doubles.is_empty() {
                return 0;
            }
            if let Some(prio) = prios.get(&doubles[0]) {
                return *prio;
            }
            return 0;
        })
        .sum()
}

fn get_prios() -> HashMap<char, u16> {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut output: HashMap<char, u16> = HashMap::new();
    let mut score = 1;
    for char in chars {
        output.insert(char, score);
        score += 1;
    }
    return output;
}
