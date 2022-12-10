use std::{collections::HashSet, hash::Hash, num};

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn len(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }

    pub fn direction(&self) -> Direction {
        let mut angle = (((-self.y as f32).atan2(self.x as f32)) * (180.0 / std::f32::consts::PI))
            .round() as i32;

        while angle < 0 {
            angle = angle + 360
        }

        //println!("x: {}, y: {}, Angle: {}", self.x, self.y, angle);
        match angle {
            0 => Direction::East,
            1..=89 => Direction::NorthEast,
            90 => Direction::North,
            91..=179 => Direction::NorthWest,
            180 => Direction::West,
            181..=269 => Direction::SouthWest,
            270 => Direction::South,
            271..=359 => Direction::SouthEast,
            _ => Direction::North,
        }
    }
}

impl Position {
    pub fn vector_from(&self, other: &Position) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn move_in_direction(&mut self, direction: &Direction) {
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        };
        self.x = self.x + dx;
        self.y = self.y + dy;
    }
}

struct Rope {
    knots: Vec<Position>,
    visited: HashSet<Position>,
}

impl Rope {
    pub fn new(x: i32, y: i32, num_knots: usize) -> Rope {
        let mut visited: HashSet<Position> = HashSet::new();
        visited.insert(Position { x, y });

        Rope {
            knots: (0..num_knots).map(|_| Position { x, y }).collect(),
            visited,
        }
    }
    /// Move the head and pull the tail with it
    pub fn move_head(&mut self, direction: &Direction) {
        self.knots[0].move_in_direction(direction);
        for i in 0..self.knots.len() - 1 {
            let vector = self.knots[i].vector_from(&self.knots[i + 1]);
            // The tail is too far away, move it
            if vector.len() > (2.0 as f32).sqrt() {
                self.knots[i + 1].move_in_direction(&vector.direction());
            }
        }
        self.visited.insert(*self.knots.last().unwrap());
    }
}

pub fn positions_visited_by_tail(input: &str, knots: usize) -> usize {
    let mut rope = Rope::new(100000, 100000, knots);

    for row in input.split("\r\n") {
        let mut split = row.split(" ");
        let dir = split.next().unwrap();
        let steps: i16 = split.next().unwrap().parse().unwrap();

        let direction = match dir {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => Direction::North,
        };
        for _ in 0..steps {
            rope.move_head(&direction);
        }
    }

    rope.visited.len()
}
