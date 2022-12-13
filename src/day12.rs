#[derive(Debug)]
struct Node {
    elevation: u32,
}

impl Node {
    pub fn parse(input: char) -> Node {
        Node {
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
    grid: Vec<Vec<Node>>,
    position: (usize, usize),
    target: (usize, usize),
}

impl Map {
    pub fn parse(input: &str) -> Map {
        let mut grid: Vec<Vec<Node>> = vec![];
        for row in input.split("\r\n") {
            grid.push(row.chars().map(Node::parse).collect());
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
            position: (0, 0),
            target: (0, 0),
        };
        map.position = map.coordinate_of(s_index);
        map.target = map.coordinate_of(e_index);

        map
    }

    fn index_of(&self, coordinate: (usize, usize)) -> usize {
        coordinate.0 * self.grid[0].len() + coordinate.1
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

pub fn calc_shortest_route_len(
    map_data: &str,
    from_coord: (usize, usize),
    to_coord: (usize, usize),
) -> usize {
    let map = Map::parse(map_data);
    0
}
