#[derive(Debug)]
struct Coord {
    elevation: u32,
}

impl Coord {
    pub fn parse(input: char) -> Coord {
        Coord {
            elevation: match input {
                'a'..='z' => input as u32,
                'S' => 'a' as u32,
                'E' => 'z' as u32,
                _ => panic!("Invalid character!"),
            },
        }
    }
}

struct Map {
    grid: Vec<Vec<Coord>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    pub fn parse(input: &str) -> Map {
        let mut grid: Vec<Vec<Coord>> = vec![];
        for row in input.split("\r\n") {
            grid.push(row.chars().map(Coord::parse).collect());
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
            grid,
            start: (0, 0),
            end: (0, 0),
        };
        map.start = map.coordinate_of(s_index);
        map.end = map.coordinate_of(e_index);
        map
    }

    fn coordinate_of(&self, index: usize) -> (usize, usize) {
        if index == 0 {
            return (0, 0);
        }
        let col_count = self.grid[0].len();
        let col = index % col_count;
        let row = (index - col) / col_count;
        (row, col)
    }
}

#[derive(Clone)]
struct PathNode {
    elevation: u32,
    h: usize,
    parent: Option<Box<PathNode>>,
    position: (usize, usize),
}

impl PathNode {
    pub fn new(
        elevation: u32,
        position: (usize, usize),
        parent: Option<Box<PathNode>>,
        h: usize,
    ) -> PathNode {
        PathNode {
            elevation,
            position,
            parent,
            h,
        }
    }

    pub fn g(&self) -> usize {
        if let Some(p) = &self.parent {
            if p.elevation > self.elevation {
                return p.g() + 100000;
            } else {
                p.g() + 1
            }
        } else {
            0
        }
    }

    pub fn f(&self) -> usize {
        self.g() + self.h
    }
}

pub fn calc_shortest_route_len(map_data: &str) -> usize {
    let map = Map::parse(map_data);
    let mut open: Vec<Box<PathNode>> = vec![];
    let mut closed: Vec<Box<PathNode>> = vec![];

    open.push(Box::new(PathNode::new(
        'a' as u32,
        map.start,
        None,
        distance_squared(map.start, map.end),
    )));

    loop {
        if open.is_empty() {
            break;
        }
        open.sort_by(|a, b| b.f().cmp(&a.f()));
        let current = open.pop().unwrap();
        closed.push(current.clone());

        if current.position == map.end {
            break;
        }

        // Adjacent coords
        for adj in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let row = current.position.0 as i32 + adj.0;
            let col = current.position.1 as i32 + adj.1;

            if row >= 0 && row < map.grid.len() as i32 && col >= 0 && col < map.grid[0].len() as i32
            {
                let adj_pos = (row as usize, col as usize);
                //let curr_el = map.grid[current.position.0][current.position.1].elevation as i32;
                let adj_el = map.grid[adj_pos.0][adj_pos.1].elevation as i32;
                /*
                if adj_el - curr_el > 1 {
                    continue;
                } */
                if let Some(_) = closed.iter().find(|n| n.position == adj_pos) {
                    continue;
                }
                if let Some(adj) = open.iter_mut().find(|n| n.position == adj_pos) {
                    if adj.g() > current.g() {
                        adj.parent = Some(current.clone());
                    }
                } else {
                    open.push(Box::new(PathNode::new(
                        adj_el as u32,
                        adj_pos,
                        Some(current.clone()),
                        distance_squared(adj_pos, map.end),
                    )));
                }
            }
        }
    }

    let mut node = closed.last().unwrap();
    let mut path: Vec<Box<PathNode>> = vec![node.clone()];
    while let Some(n) = &node.parent {
        node = n;
        path.push(node.clone());
    }
    path.reverse();

    for node in &path {
        println!("{}:{}", node.position.0 + 1, node.position.1 + 1);
    }

    path.len() - 1
}

fn distance_squared(from: (usize, usize), to: (usize, usize)) -> usize {
    ((to.0 as i16 - from.0 as i16).pow(2) + (to.1 as i16 - from.1 as i16).pow(2)) as usize
    //((from.0 as i16 - to.0 as i16).abs() + (from.1 as i16 - to.1 as i16).abs()) as usize
}
