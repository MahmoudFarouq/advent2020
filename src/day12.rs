use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn origin() -> Self {
        Point::new(0.0, 0.0)
    }

    fn step(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    fn manhattan(&self) -> f32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Vector {
    angle: f32,
    position: Point,
}

impl Vector {
    fn new(position: Point) -> Self {
        Self {
            angle: 0.0,
            position,
        }
    }

    fn origin() -> Self {
        Self::new(Point::origin())
    }

    fn rotate(&mut self, angle: f32) {
        self.angle += angle;
    }

    fn forward(&mut self, distance: f32) {
        self.position.step(
            distance * self.angle.to_radians().cos(),
            distance * self.angle.to_radians().sin(),
        )
    }

    fn north(&mut self, distance: f32) {
        self.position.step(0.0, distance)
    }

    fn east(&mut self, distance: f32) {
        self.position.step(distance, 0.0)
    }

    fn south(&mut self, distance: f32) {
        self.position.step(0.0, -distance)
    }

    fn west(&mut self, distance: f32) {
        self.position.step(-distance, 0.0)
    }

    fn rotate_around_origin(&mut self, angle: f32) {
        let old_pos = self.position;

        self.position.x =
            angle.to_radians().cos() * old_pos.x - angle.to_radians().sin() * old_pos.y;

        self.position.y =
            angle.to_radians().sin() * old_pos.x + angle.to_radians().cos() * old_pos.y;
    }
}

#[derive(Debug, Clone)]
enum Command {
    NORTH(f32),
    EAST(f32),
    SOUTH(f32),
    WEST(f32),
    LEFT(f32),
    RIGHT(f32),
    FORWARD(f32),
}

impl Command {
    fn from(command_str: &str) -> Self {
        match command_str.split_at(1) {
            ("N", value) => Self::NORTH(value.parse().unwrap()),
            ("E", value) => Self::EAST(value.parse().unwrap()),
            ("S", value) => Self::SOUTH(value.parse().unwrap()),
            ("W", value) => Self::WEST(value.parse().unwrap()),
            ("L", value) => Self::LEFT(value.parse().unwrap()),
            ("R", value) => Self::RIGHT(value.parse().unwrap()),
            ("F", value) => Self::FORWARD(value.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Command> {
    input.lines().map(|line| Command::from(line)).collect()
}

#[aoc(day12, part1)]
fn day12_part1(input: &str) -> Option<isize> {
    let commands = parse(input);
    let mut vector = Vector::origin();
    for command in commands.iter() {
        match command {
            Command::NORTH(value) => vector.north(*value),
            Command::EAST(value) => vector.east(*value),
            Command::SOUTH(value) => vector.south(*value),
            Command::WEST(value) => vector.west(*value),

            Command::LEFT(value) => vector.rotate(*value),
            Command::RIGHT(value) => vector.rotate(-*value),

            Command::FORWARD(value) => vector.forward(*value),
        }
    }
    Some(vector.position.manhattan().round() as isize)
}

#[aoc(day12, part2)]
fn day12_part2(input: &str) -> Option<isize> {
    let commands = parse(input);
    let mut ship = Vector::origin();
    let mut waypoint = Vector::new(Point::new(10_f32, 1_f32));

    for command in commands.iter() {
        match command {
            Command::NORTH(value) => waypoint.north(*value),
            Command::EAST(value) => waypoint.east(*value),
            Command::SOUTH(value) => waypoint.south(*value),
            Command::WEST(value) => waypoint.west(*value),

            Command::LEFT(value) => waypoint.rotate_around_origin(*value),
            Command::RIGHT(value) => waypoint.rotate_around_origin(-*value),

            Command::FORWARD(value) => {
                ship.position.x += *value * waypoint.position.x;
                ship.position.y += *value * waypoint.position.y;
            }
        }
    }
    Some(ship.position.manhattan().round() as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(day12_part1(input), Some(25));
    }

    #[test]
    fn test_part2() {
        let input = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(day12_part2(input), Some(286));
    }
}
