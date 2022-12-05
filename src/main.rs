use std::fs;

mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("Day 2: {:?}", solve_day2());
    println!("Day 3: {:?}", solve_day3());
    println!("Day 4: {:?}", solve_day4());
    println!("Day 5: {:?}", solve_day5());
}

fn solve_day2() -> (u16, u16) {
    let input = fs::read_to_string("day2.txt").expect("Should have been able to read day2.txt");
    (
        day2::calculate_total_score(&input, false).expect("Day 2 should have worked"),
        day2::calculate_total_score(&input, true).expect("Day 2 should have worked"),
    )
}

fn solve_day3() -> (u16, u16) {
    let input = fs::read_to_string("day3.txt").expect("Should have been able to read day3.txt");
    (
        day3::calc_doublet_priorities(&input),
        day3::calc_badge_priorities(&input),
    )
}

fn solve_day4() -> (u16, u16) {
    let input = fs::read_to_string("day4.txt").expect("Should have been able to read day4.txt");
    (
        day4::count_fully_overlapping(&input),
        day4::count_overlapping(&input),
    )
}

fn solve_day5() -> (String, String) {
    let input = fs::read_to_string("day5.txt").expect("Should have been able to read day5.txt");
    (
        day5::parse_instructions(&input, false),
        day5::parse_instructions(&input, true),
    )
}
