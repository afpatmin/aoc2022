use std::rc::Rc;

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
                _ => 0,
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
    h: usize,
    parent: Option<Rc<PathNode>>,
    position: (usize, usize),
}

impl PathNode {
    pub fn new(position: (usize, usize), parent: Option<Rc<PathNode>>, h: usize) -> PathNode {
        PathNode {
            position,
            parent,
            h,
        }
    }

    pub fn g(&self) -> usize {
        if let Some(p) = &self.parent {
            p.g() + 1
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
    let mut open: Vec<Rc<PathNode>> = vec![];
    let mut closed: Vec<Rc<PathNode>> = vec![];

    open.push(Rc::new(PathNode::new(
        map.start,
        None,
        distance_squared(map.start, map.end),
    )));

    loop {
        open.sort_by(|a, b| b.f().cmp(&a.f()));
        if open.is_empty() {
            break;
        }
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
                let current_elevation =
                    map.grid[current.position.0][current.position.1].elevation as i32;
                let adjacent_elevation = map.grid[adj_pos.0][adj_pos.1].elevation as i32;
                if let Some(_) = closed.iter().find(|n| n.position == adj_pos) {
                    continue;
                }
                if adjacent_elevation - current_elevation > 1 {
                    continue;
                }

                let new_node = Rc::new(PathNode::new(
                    adj_pos,
                    Some(current.clone()),
                    distance_squared(adj_pos, map.end),
                ));

                if let Some(i) = open.iter().position(|n| n.position == adj_pos) {
                    if open[i].g() < current.g() {
                        open[i] = new_node;
                    }
                } else {
                    open.push(new_node);
                }
            }
        }
    }

    let mut node = closed.last().unwrap();
    let mut path: Vec<Rc<PathNode>> = vec![node.clone()];
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
