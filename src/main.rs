use std::fs;

mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    println!("Day 2: {:?}", solve_day2());
    println!("Day 3: {:?}", solve_day3());
    println!("Day 4: {:?}", solve_day4());
    println!("Day 5: {:?}", solve_day5());
    println!("Day 6: {:?}", solve_day6());
    println!("Day 7: {:?}", solve_day7());
    println!("Day 8: {:?}", solve_day8());
    println!("Day 9: {:?}", solve_day9());
    println!("Day 10: {:?}", solve_day10());
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

fn solve_day6() -> (usize, usize) {
    let input = fs::read_to_string("day6.txt").expect("Should have been able to read day6.txt");
    (
        day6::find_marker_index(&input, day6::MarkerType::Packet).expect("Start index not found"),
        day6::find_marker_index(&input, day6::MarkerType::Message).expect("Start index not found"),
    )
}

fn solve_day7() -> (usize, usize) {
    let input = fs::read_to_string("day7.txt").expect("Should have been able to read day7.txt");
    let filesystem = day7::FileSystem::from_commands(&input);
    (
        day7::sum_bytesizes(&filesystem.root, 100000, 0),
        day7::find_directory_to_delete(&filesystem.root, 70000000, 30000000),
    )
}

fn solve_day8() -> (usize, usize) {
    let input = fs::read_to_string("day8.txt").expect("Should have been able to read day8.txt");
    (
        day8::visible_trees_in(&input),
        day8::highest_scenic_score(&input),
    )
}

fn solve_day9() -> (usize, usize) {
    let input = fs::read_to_string("day9.txt").expect("Should have been able to read day9.txt");
    (
        day9::positions_visited_by_tail(&input, 2),
        day9::positions_visited_by_tail(&input, 10),
    )
}

fn solve_day10() -> (usize, usize) {
    let input = fs::read_to_string("day10.txt").expect("Should have been able to read day10.txt");
    (day10::sum_signal_strengths(&input, 20, 40), 0)
}
