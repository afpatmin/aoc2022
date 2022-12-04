use std::fs;

mod day2;
mod day3;

fn main() {
    println!("Day 2: {:?}", day_2());
    println!("Day 3: {:?}", day_3());
}

fn day_2() -> (u16, u16) {
    let input = fs::read_to_string("day2.txt").expect("Should have been able to read day2.txt");
    (
        day2::calculate_total_score(&input, false).expect("Day 2 should have worked"),
        day2::calculate_total_score(&input, true).expect("Day 2 should have worked"),
    )
}

fn day_3() -> (u16, u16) {
    let input = fs::read_to_string("day3.txt").expect("Should have been able to read day3.txt");
    (
        day3::calc_doublet_priorities(&input),
        day3::calc_badge_priorities(&input),
    )
}
