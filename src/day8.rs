use std::collections::HashSet;
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    row: usize,
    col: usize,
    height: i16,
}

pub fn visible_trees_in(input: &str) -> usize {
    let coords = parse_input(input);
    if let Some(last_row) = coords.last() {
        if let Some(last_col) = last_row.last() {
            let num_rows = last_col.row;
            let num_cols = last_col.col;
            let mut visible_trees: HashSet<Coord> = HashSet::new();

            for row in 0..num_rows {
                visible_trees.extend(visible_in(&coords[row]));
                let mut reversed = coords[row].clone();
                reversed.reverse();
                visible_trees.extend(visible_in(&reversed));
            }

            for c in 0..=num_cols {
                let mut col: Vec<Coord> = vec![];
                for r in 0..=num_rows {
                    col.push(coords[r][c]);
                }
                let mut reversed = col.clone();
                reversed.reverse();
                visible_trees.extend(visible_in(&col));
                visible_trees.extend(visible_in(&reversed));
            }
            return visible_trees.len();
        }
    }
    return 0;
}

fn visible_in(line: &Vec<Coord>) -> Vec<Coord> {
    let mut output: Vec<Coord> = vec![];
    let mut highest_tree = -1;
    for tree in line.iter() {
        if tree.height > highest_tree {
            highest_tree = tree.height;
            output.push(*tree);
        }
    }
    output
}

pub fn highest_scenic_score(input: &str) -> usize {
    let coords = parse_input(input);

    let mut max_score = 0;
    for row in &coords {
        for coord in row {
            let score = scenic_score_of(coord, &coords);
            if score >= max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn scenic_score_of(coord: &Coord, coords: &Vec<Vec<Coord>>) -> usize {
    let mut distances: Vec<u8> = vec![0, 0, 0, 0];

    // North
    if coord.row > 0 {
        if let Some(row) = coords.get(coord.row - 1) {
            if let Some(compare) = row.get(coord.col) {
                distances[0] = sum_distance(coord, compare, coords, 0);
            }
        }
    }
    // West
    if let Some(row) = coords.get(coord.row) {
        if coord.col > 0 {
            if let Some(compare) = row.get(coord.col - 1) {
                distances[1] = sum_distance(coord, compare, coords, 0);
            }
        }
    }
    // South
    if let Some(row) = coords.get(coord.row + 1) {
        if let Some(compare) = row.get(coord.col) {
            distances[2] = sum_distance(coord, compare, coords, 0);
        }
    }
    // East
    if let Some(row) = coords.get(coord.row) {
        if let Some(compare) = row.get(coord.col + 1) {
            distances[3] = sum_distance(coord, compare, coords, 0);
        }
    }
    distances.iter().fold(1, |acc, i| acc * *i as usize)
}

fn sum_distance(
    coord: &Coord,
    compare_coord: &Coord,
    coords: &Vec<Vec<Coord>>,
    accumulated: u8,
) -> u8 {
    if coord.height <= compare_coord.height {
        return accumulated + 1;
    } else {
        let mut move_x = compare_coord.col as i16 - coord.col as i16;
        if move_x != 0 {
            move_x = match move_x.is_negative() {
                true => -1,
                false => 1,
            }
        }

        let mut move_y = compare_coord.row as i16 - coord.row as i16;
        if move_y != 0 {
            move_y = match move_y.is_negative() {
                true => -1,
                false => 1,
            }
        }

        let row = compare_coord.row as i16 + move_y;
        let col = compare_coord.col as i16 + move_x;

        if row >= 0 && col >= 0 {
            if let Some(next_row) = coords.get(row as usize) {
                if let Some(next_coord) = next_row.get(col as usize) {
                    return sum_distance(coord, next_coord, coords, accumulated) + 1;
                }
            }
        }

        return accumulated + 1;
    }
}

fn parse_input(input: &str) -> Vec<Vec<Coord>> {
    let mut rows: Vec<Vec<Coord>> = vec![];
    let str_rows: Vec<&str> = input.split("\r\n").collect();
    for row in 0..str_rows.len() {
        rows.push(vec![]);
        let cols: Vec<char> = str_rows[row].chars().collect();
        for col in 0..cols.len() {
            rows[row].push(Coord {
                row,
                col,
                height: cols[col].to_digit(10).unwrap() as i16,
            });
        }
    }
    rows
}
