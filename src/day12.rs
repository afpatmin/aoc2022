struct Coord {
    row: usize,
    col: usize,
}

struct Map {
    height_map: Vec<Vec<usize>>,
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
                        'a'..='z' => c as usize,
                        'S' => 'a' as usize,
                        'E' => 'z' as usize,
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
            height_map,
            start: Coord { row: 0, col: 0 },
            end: Coord { row: 0, col: 0 },
        };
        fn coord(index: usize, col_count: usize) -> Coord {
            let col = index % col_count;
            let row = (index - col) / col_count;
            Coord { row, col }
        }
        let col_count = map.height_map[0].len();
        map.start = coord(s_index, col_count);
        map.end = coord(e_index, col_count);
        map
    }
}

struct Node {
    g: usize,
    h: usize,
    parent: Option<Box<Node>>,
}

impl Node {
    pub fn new(g: usize, pos: &Coord, end: &Coord, parent: Option<Box<Node>>) -> Node {
        Node {
            g,
            h: (pos.row.abs_diff(end.row)).pow(2) + (pos.col.abs_diff(end.col)).pow(2),
            parent: None,
        }
    }

    pub fn f(&self) -> usize {
        self.g + self.h
    }
}

pub fn pathfinder(map_data: &str) -> usize {
    let map = Map::parse(map_data);

    let current = Node::new(0, &map.start, &map.end, None);

    0
}
