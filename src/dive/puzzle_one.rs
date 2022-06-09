#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    amount: u32,
}

#[derive(Default)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { horizontal: 0, depth: 0 }
    }

    pub fn apply_movement(&mut self, movement: Movement) {
        match movement.direction {
            Direction::Down => self.depth += movement.amount as i32,
            Direction::Forward => self.horizontal += movement.amount as i32,
            Direction::Up => self.depth -= movement.amount as i32,
        }
    }
}

fn parse(lines: &Vec<String>) -> Vec<Movement> {
    lines
        .into_iter()
        .map(|line| line.split(" ").collect())
        .map(|movement_and_direction: Vec<&str>| Movement {
            direction: match movement_and_direction[0] {
                "up" => Direction::Up,
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                _ => panic!("Failed to parse direction."),
            },
            amount: movement_and_direction[1]
                .parse()
                .expect("Failed to parse number from string."),
        })
        .collect()
}

pub fn puzzle_one(lines: &Vec<String>) -> i32 {
    let mut position = Position::new();
    for movement in parse(lines) {
        position.apply_movement(movement);
    }
    position.horizontal * position.depth
}
