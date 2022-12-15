use std::{collections::HashSet, iter::successors};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

struct Map {
    heights: Vec<Vec<usize>>,
    start: Coord,
    end: Coord,
}

impl Map {
    pub fn parse(input: &str) -> Map {
        let mut height_map: Vec<Vec<usize>> = vec![];
        for row in input.split("\r\n") {
            height_map.push(
                row.chars()
                    .map(|c| match c {
                        'a'..='z' => c as usize - 97,
                        'S' => 'a' as usize - 97,
                        'E' => 'z' as usize - 97,
                        _ => panic!("Invalid character!"),
                    })
                    .collect(),
            );
        }
        let s_index = input
            .replace("\r\n", "")
            .chars()
            .position(|c| c == 'S')
            .unwrap();
        let e_index = input
            .replace("\r\n", "")
            .chars()
            .position(|c| c == 'E')
            .unwrap();
        let mut map = Map {
            heights: height_map,
            start: Coord { row: 0, col: 0 },
            end: Coord { row: 0, col: 0 },
        };
        fn coord(index: usize, col_count: usize) -> Coord {
            let col = index % col_count;
            let row = (index - col) / col_count;
            Coord { row, col }
        }
        let col_count = map.heights[0].len();
        map.start = coord(s_index, col_count);
        map.end = coord(e_index, col_count);
        map
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Node {
    f: usize,
    g: usize,
    h: usize,
    pos: Coord,
    adjacent: Vec<Coord>,
    parent: Option<Box<Node>>,
}

impl Node {
    pub fn new(pos: Coord, map: &Map, adjacent: Vec<Coord>) -> Node {
        Node {
            f: usize::MAX,
            g: usize::MAX,
            h: (pos.row.abs_diff(map.end.row)).pow(2) + (pos.col.abs_diff(map.end.col)).pow(2),
            pos,
            adjacent: adjacent,
            parent: None,
        }
    }

    fn find_adjacent(pos: Coord, map: &Map) -> Vec<Coord> {
        let mut output: Vec<Coord> = vec![];
        let node_elevation = map.heights[pos.row][pos.col] as i32;
        for c in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let adj_row = pos.row as i32 + c.0;
            let adj_col = pos.col as i32 + c.1;
            if adj_row >= 0
                && adj_row < map.heights.len() as i32
                && adj_col >= 0
                && adj_col < map.heights[0].len() as i32
            {
                let adj_elevation = map.heights[adj_row as usize][adj_col as usize] as i32;
                if adj_elevation - node_elevation <= 1 {
                    output.push(Coord {
                        row: adj_row as usize,
                        col: adj_col as usize,
                    });
                }
            }
        }
        output
    }
}

pub fn pathfinder(map_data: &str) -> usize {
    let map = Map::parse(map_data);

    let mut counter = 0;
    let mut open_set: HashSet<Node> = HashSet::new();
    let mut closed_set: HashSet<Node> = HashSet::new();

    let mut current = Node::new(map.start, &map, Node::find_adjacent(map.start, &map));
    current.f = 0;
    current.g = 0;
    open_set.insert(current.clone());

    loop {
        if open_set.is_empty() {
            break;
        }
        current = open_set
            .iter()
            .reduce(|accum, item| {
                if accum.f < item.f || (accum.f == item.f && accum.g < item.g) {
                    accum
                } else {
                    item
                }
            })
            .unwrap()
            .clone();
        if current.pos == map.end {
            break;
        }
        open_set.remove(&current);

        for adj_coord in &current.adjacent {
            let old_adj = Node::new(*adj_coord, &map, Node::find_adjacent(*adj_coord, &map));
            let mut adj = old_adj.clone();
            adj.g = current.g + 1;
            adj.f = adj.g + adj.h;
            adj.parent = Some(Box::new(current.clone()));

            if let None = open_set
                .iter()
                .find(|item| item.pos == adj.pos && item.f < adj.f)
            {
                if let None = closed_set
                    .iter()
                    .find(|item| item.pos == adj.pos && item.f < adj.f)
                {
                    open_set.remove(&old_adj);
                    open_set.insert(adj);
                }
            }
        }
        closed_set.insert(current);

        counter = counter + 1;
    }

    0
}
