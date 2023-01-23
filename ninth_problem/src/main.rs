use std::cmp::Ordering;
use std::collections::HashSet;

mod variable;

struct Knot {
    cordinates: Cordinates,
    previous_position: Cordinates,
    history: HashSet<Cordinates>,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        let mut history = HashSet::new();
        let cordinates = Cordinates::new(x, y);
        history.insert(cordinates);
        Self {
            cordinates,
            previous_position: cordinates,
            history,
        }
    }
    fn move_head(&mut self, direction: &Direction) {
        self.previous_position = self.cordinates;
        match direction {
            Direction::Down => self.cordinates.y -= 1,
            Direction::Up => self.cordinates.y += 1,
            Direction::Left => self.cordinates.x -= 1,
            Direction::Right => self.cordinates.x += 1,
        }
    }

    fn update_knot(&mut self, previous_knot_cordinates: Cordinates) {
        if !self.is_previous_knot_close(previous_knot_cordinates) {
            self.previous_position = self.cordinates;
            self.cordinates.x += match previous_knot_cordinates.x.cmp(&self.cordinates.x) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            self.cordinates.y += match previous_knot_cordinates.y.cmp(&self.cordinates.y) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            self.history.insert(self.cordinates);
        }
    }

    fn is_previous_knot_close(&self, previous_knot_cordinates: Cordinates) -> bool {
        (previous_knot_cordinates.x - self.cordinates.x < 2
            && previous_knot_cordinates.x - self.cordinates.x > -2)
            && (previous_knot_cordinates.y - self.cordinates.y < 2
                && previous_knot_cordinates.y - self.cordinates.y > -2)
    }
}

struct Insturction {
    direction: Direction,
    steps: i32,
}

impl Insturction {
    fn new(direction: &str, steps: i32) -> Self {
        let direction = match direction {
            "D" => Direction::Down,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "R" => Direction::Right,
            _ => panic!("invalid direction: {:?}", direction),
        };
        Self { direction, steps }
    }
}

enum Direction {
    Down,
    Left,
    Up,
    Right,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Cordinates {
    x: i32,
    y: i32,
}

impl Cordinates {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let mut snake = create_snake(10);
    let input = variable::INPUT;
    let input: Vec<Insturction> = input
        .split('\n')
        .map(|line| {
            let instruction: Vec<&str> = line.split(' ').collect();
            Insturction::new(instruction[0], instruction[1].parse().unwrap())
        })
        .collect();
    move_snake(&mut snake, input);
    println!("the solution is {:?}", snake[snake.len() - 1].history.len());
}

fn create_snake(length: i32) -> Vec<Knot> {
    let mut result = Vec::new();
    for _ in 0..length {
        result.push(Knot::new(0, 0))
    }
    result
}

fn move_snake(snake: &mut Vec<Knot>, instructions: Vec<Insturction>) {
    for instruction in instructions {
        for _ in 0..instruction.steps {
            for i in 0..snake.len() {
                if i == 0 {
                    snake[i].move_head(&instruction.direction);
                    continue;
                }
                let previous_knot_cordinates = snake[i - 1].cordinates;
                snake[i].update_knot(previous_knot_cordinates);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn how_many_unique_steps() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let input: Vec<Insturction> = input
            .split('\n')
            .map(|line| {
                let instruction: Vec<&str> = line.split(' ').collect();
                Insturction::new(instruction[0], instruction[1].parse().unwrap())
            })
            .collect();
        let mut snake = create_snake(2);
        move_snake(&mut snake, input);
        let result = snake[snake.len() - 1].history.len();
        let expected = 13;
        assert_eq!(result, expected);
    }
    #[test]
    fn same_instruction_but_ten_knots() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let input: Vec<Insturction> = input
            .split('\n')
            .map(|line| {
                let instruction: Vec<&str> = line.split(' ').collect();
                Insturction::new(instruction[0], instruction[1].parse().unwrap())
            })
            .collect();
        let mut snake = create_snake(10);
        move_snake(&mut snake, input);
        let result = snake[snake.len() - 1].history.len();
        let expected = 1;
        assert_eq!(result, expected);
    }
    #[test]
    fn how_many_unique_steps_9_knots() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let input: Vec<Insturction> = input
            .split('\n')
            .map(|line| {
                let instruction: Vec<&str> = line.split(' ').collect();
                Insturction::new(instruction[0], instruction[1].parse().unwrap())
            })
            .collect();
        let mut snake = create_snake(10);
        move_snake(&mut snake, input);
        let result = snake[snake.len() - 1].history.len();
        let expected = 36;
        assert_eq!(result, expected);
    }
}
