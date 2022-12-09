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
