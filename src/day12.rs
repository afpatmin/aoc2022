#[derive(Clone, Debug, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

struct Map {
    chars: Vec<Vec<char>>,
    heights: Vec<Vec<usize>>,
    start: Coord,
    end: Coord,
}

impl Map {
    pub fn parse(input: &str) -> Map {
        let mut heights: Vec<Vec<usize>> = vec![];
        let mut chars: Vec<Vec<char>> = vec![];
        for row in input.split("\r\n") {
            chars.push(row.chars().collect());
            heights.push(
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
            chars,
            heights,
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

    pub fn mark_node(&mut self, coord: &Coord, symbol: char) {
        self.chars[coord.row][coord.col] = symbol;
    }

    pub fn print(&self) {
        for row in &self.chars {
            for char in row {
                print!("{}", char);
            }
            print!("\n");
        }
    }
}

#[derive(Clone)]
struct Node {
    f: f32,
    g: usize,
    h: f32,
    pos: Coord,
    adjacent: Vec<Coord>,
    parent: Option<Box<Node>>,
}

impl Node {
    pub fn new(pos: Coord, map: &Map, adjacent: Vec<Coord>) -> Node {
        Node {
            f: f32::MAX,
            g: usize::MAX,
            h: ((pos.row.abs_diff(map.end.row).pow(2) + (pos.col.abs_diff(map.end.col)).pow(2))
                as f32)
                .sqrt(),
            pos,
            adjacent: adjacent,
            parent: None,
        }
    }

    fn find_neighbors(pos: &Coord, map: &Map) -> Vec<Coord> {
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
    let mut map = Map::parse(map_data);

    let mut counter = 0;
    let mut open_set: Vec<Node> = vec![];
    let mut closed_set: Vec<Node> = vec![];

    let mut current = Node::new(
        map.start.clone(),
        &map,
        Node::find_neighbors(&map.start, &map),
    );
    current.f = 0.0;
    current.g = 0;
    open_set.push(current.clone());

    loop {
        if open_set.is_empty() {
            break;
        }
        open_set.sort_by(|a, b| b.f.total_cmp(&a.f));
        current = open_set.pop().unwrap();
        closed_set.push(current.clone());

        if current.pos == map.end {
            break;
        }

        for adj_coord in &current.adjacent {
            let mut adj = Node::new(
                adj_coord.clone(),
                &map,
                Node::find_neighbors(adj_coord, &map),
            );
            adj.parent = Some(Box::new(current.clone()));

            if let None = closed_set.iter().find(|item| item.pos == adj.pos) {
                if let None = open_set
                    .iter()
                    .find(|item| item.pos == adj.pos && item.f < adj.f)
                {
                    adj.g = current.g + 1;
                    adj.f = adj.g as f32 + adj.h;
                    open_set.push(adj);
                }
            }
        }

        counter = counter + 1;
    }

    let mut path: Vec<Coord> = vec![];
    while let Some(parent) = current.parent {
        path.push(current.pos);
        current = *parent;
    }
    path.reverse();
    for node in 0..path.len() - 1 {
        if node == 0 {
            map.mark_node(&path[node], '-');
        } else {
            if path[node + 1].row > path[node].row {
                map.mark_node(&path[node], '\u{2193}');
            } else if path[node + 1].row < path[node].row {
                map.mark_node(&path[node], '\u{2191}');
            } else if path[node + 1].col < path[node].col {
                map.mark_node(&path[node], '\u{2190}');
            } else if path[node + 1].col > path[node].col {
                map.mark_node(&path[node], '\u{2192}');
            }
        }
    }
    map.print();
    path.len()
}
