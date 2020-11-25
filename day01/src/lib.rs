use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Movement {
    turn: Turn,
    steps: i32,
}

impl Movement {
    fn new(turn: Turn, steps: i32) -> Self {
        Self { turn, steps }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Self {
        match (self, turn) {
            (Self::North, Turn::Right) => Self::East,
            (Self::East, Turn::Right) => Self::South,
            (Self::South, Turn::Right) => Self::West,
            (Self::West, Turn::Right) => Self::North,
            (Self::North, Turn::Left) => Self::West,
            (Self::West, Turn::Left) => Self::South,
            (Self::South, Turn::Left) => Self::East,
            (Self::East, Turn::Left) => Self::North,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Coordinates {
    pos_x: i32,
    pos_y: i32,
}

impl Coordinates {
    fn origin_distance(&self) -> i32 {
        self.pos_x.abs() + self.pos_y.abs()
    }

    fn step(&self, direction: &Direction) -> Self {
        self.jump(direction, 1)
    }

    fn jump(&self, direction: &Direction, steps: i32) -> Self {
        let (pos_x, pos_y) = match direction {
            Direction::North => (self.pos_x, self.pos_y - steps),
            Direction::East => (self.pos_x + steps, self.pos_y),
            Direction::South => (self.pos_x, self.pos_y + steps),
            Direction::West => (self.pos_x - steps, self.pos_y),
        };
        Self { pos_x, pos_y }
    }
}

#[derive(Debug, Default)]
struct Position {
    direction: Direction,
    coordinates: Coordinates,
}

impl Position {
    fn coordinates(&self) -> Coordinates {
        self.coordinates
    }

    fn origin_distance(&self) -> i32 {
        self.coordinates.origin_distance()
    }

    fn jump(&self, movement: &Movement) -> Self {
        let direction = self.direction.turn(&movement.turn);
        let coordinates = self.coordinates.jump(&direction, movement.steps);
        Self {
            direction,
            coordinates,
        }
    }

    fn walk(&self, movement: &Movement) -> impl Iterator<Item = Coordinates> {
        let direction = self.direction.turn(&movement.turn);
        let start = self.coordinates();
        (0..movement.steps).scan(start, move |coord, _| {
            *coord = coord.step(&direction);
            Some(*coord)
        })
    }
}

pub fn part1(movements: &[Movement]) -> i32 {
    let mut position = Position::default();
    for movement in movements {
        position = position.jump(movement);
    }
    position.origin_distance()
}

pub fn part2(movements: &[Movement]) -> Option<i32> {
    let mut position = Position::default();
    let mut visited = HashSet::new();
    visited.insert(position.coordinates());

    for movement in movements {
        for coordinates in position.walk(movement) {
            if visited.contains(&coordinates) {
                return Some(coordinates.origin_distance());
            }
            visited.insert(coordinates);
        }
        position = position.jump(movement);
    }
    None
}

impl TryFrom<char> for Turn {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch.to_ascii_uppercase() {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(format!("Invalid turn: {}", ch)),
        }
    }
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ch = s
            .chars()
            .next()
            .ok_or_else(|| "Invalid movement: empty string".to_string())?;
        let turn = Turn::try_from(ch)?;
        let steps = s
            .get(1..)
            .unwrap()
            .parse::<i32>()
            .map_err(|err| format!("Invalid number of steps: {}", err))?;
        Ok(Self::new(turn, steps))
    }
}
