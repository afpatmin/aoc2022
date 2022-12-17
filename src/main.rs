use std::fs;

mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    for n in 13..=13 {
        solve_day(n);
    }
}

fn solve_day(n: u8) {
    let input = fs::read_to_string(format!("data/day{}.txt", n))
        .expect("Should have been able to read day[n].txt");
    println!("\n*** Day {} ***", n);
    match n {
        2 => {
            println!(
                "a): {}\nb): {}",
                day2::calculate_score(&input, false).expect("Day 2 should have worked"),
                day2::calculate_score(&input, true).expect("Day 2 should have worked")
            );
        }
        3 => {
            println!(
                "a): {}\nb): {}",
                day3::calc_doublet_priorities(&input),
                day3::calc_badge_priorities(&input)
            );
        }
        4 => {
            println!(
                "a): {}\nb): {}",
                day4::count_fully_overlapping(&input),
                day4::count_overlapping(&input)
            );
        }
        5 => {
            println!(
                "a): {}\nb): {}",
                day5::parse_instructions(&input, false),
                day5::parse_instructions(&input, true)
            );
        }
        6 => {
            println!(
                "a): {}\nb): {}",
                day6::find_marker_index(&input, day6::MarkerType::Packet)
                    .expect("Start index not found"),
                day6::find_marker_index(&input, day6::MarkerType::Message)
                    .expect("Start index not found")
            );
        }
        7 => {
            let filesystem = day7::FileSystem::from_commands(&input);
            println!(
                "a): {}\nb): {}",
                day7::sum_bytesizes(&filesystem.root, 100000, 0),
                day7::find_directory_to_delete(&filesystem.root, 70000000, 30000000)
            );
        }
        8 => {
            println!(
                "a): {}\nb): {}",
                day8::visible_trees_in(&input),
                day8::highest_scenic_score(&input)
            );
        }
        9 => {
            println!(
                "a): {}\nb): {}",
                day9::positions_visited_by_tail(&input, 2),
                day9::positions_visited_by_tail(&input, 10)
            );
        }
        10 => {
            println!("a):");
            day10::render_frame(&input);
            println!("b): {}", day10::sum_signal_strengths(&input, 20, 40));
        }
        11 => {
            println!(
                "a): {}\nb): {}",
                day11::evaluate_monkey_business_level(&input, 20, 3),
                day11::evaluate_monkey_business_level(&input, 10000, 1),
            );
        }
        12 => {
            println!(
                "a): {}\nb): {}",
                day12::find_path_from_map_start(&input),
                day12::find_shortest_hike(&input)
            );
        }
        13 => {
            println!("a):{}", day13::count_valid_packet_pairs(&input));
        }
        14 => {}
        15 => {}
        16 => {}
        17 => {}
        18 => {}
        19 => {}
        20 => {}
        21 => {}
        22 => {}
        23 => {}
        24 => {}
        25 => {}
        _ => {}
    }
}
