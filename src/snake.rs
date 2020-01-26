use common::{Position, Positionable, WORLD_HEIGHT, WORLD_WIDTH};
use edibles::Edible;

pub const SNAKE_PART_WIDTH: f64 = 10.0;
pub const SNAKE_PART_HALF_WIDTH: f64 = 5.0;

const INITIAL_SNAKE_BODY_SIZE: u32 = 10;

#[derive(Debug)]
pub struct Snake {
    pub parts: Vec<SnakePart>,
    pub current_direction: MoveDirection,
    pub alive: bool,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            parts: (0..INITIAL_SNAKE_BODY_SIZE)
                .map(|x| SnakePart {
                    position: Position {
                        x: (WORLD_WIDTH / 2.0) - (f64::from(x) * SNAKE_PART_WIDTH),
                        y: (WORLD_HEIGHT / 2.0),
                    },
                    index: x,
                })
                .collect::<Vec<SnakePart>>(),
            current_direction: MoveDirection::RIGHT,
            alive: true,
        }
    }

    pub fn update(&mut self, edibles: &mut Vec<Edible>) {
        let old_x = self.parts[0].position.x;
        let old_y = self.parts[0].position.y;

        match self.current_direction {
            MoveDirection::LEFT => {
                let tmp_x = self.parts[0].position.x - SNAKE_PART_WIDTH;
                let new_x = if tmp_x >= 0.0 {
                    tmp_x
                } else {
                    WORLD_WIDTH - SNAKE_PART_WIDTH
                };
                self.parts[0].position.x = new_x;
            }
            MoveDirection::RIGHT => {
                let tmp_x = self.parts[0].position.x + SNAKE_PART_WIDTH;
                let new_x = if tmp_x < WORLD_WIDTH { tmp_x } else { 0.0 };
                self.parts[0].position.x = new_x;
            }
            MoveDirection::UP => {
                let tmp_y = self.parts[0].position.y - SNAKE_PART_WIDTH;
                let new_y = if tmp_y >= 0.0 {
                    tmp_y
                } else {
                    WORLD_HEIGHT - SNAKE_PART_WIDTH
                };
                self.parts[0].position.y = new_y;
            }
            MoveDirection::DOWN => {
                let tmp_y = self.parts[0].position.y + SNAKE_PART_WIDTH;
                let new_y = if tmp_y < WORLD_HEIGHT { tmp_y } else { 0.0 };
                self.parts[0].position.y = new_y;
            }
        }
        self.check_for_intersection(edibles);
        self.update_body(old_x, old_y);
    }

    fn check_for_intersection(&mut self, edibles: &mut Vec<Edible>) {
        let (head_x, head_y) = (self.parts[0].position.x, self.parts[0].position.y);

        for part in &self.parts[1..] {
            if head_x == part.position.x && head_y == part.position.y {
                self.alive = false;
            }
        }

        for e in edibles {
            if head_x + SNAKE_PART_HALF_WIDTH >= e.position().x - SNAKE_PART_HALF_WIDTH
                && head_x - SNAKE_PART_HALF_WIDTH < e.position().x + SNAKE_PART_HALF_WIDTH
                && head_y + SNAKE_PART_HALF_WIDTH >= e.position().y - SNAKE_PART_HALF_WIDTH
                && head_y - SNAKE_PART_HALF_WIDTH < e.position().y + SNAKE_PART_HALF_WIDTH
            {
                e.eat(self);
            }
        }
    }

    pub fn grow(&mut self) {
        let (tail_x, tail_y) = (
            self.parts.last().unwrap().position.x,
            self.parts.last().unwrap().position.y,
        );
        let (x, y) = match self.current_direction {
            MoveDirection::LEFT => (tail_x + 1.0, tail_y),
            MoveDirection::RIGHT => (tail_x - 1.0, tail_y),
            MoveDirection::UP => (tail_x, tail_y - 1.0),
            MoveDirection::DOWN => (tail_x, tail_y + 1.0),
        };

        let index = self.parts.last().unwrap().index + 1;

        self.parts.push(SnakePart {
            index: index,
            position: Position { x: x, y: y },
        });
    }

    pub fn shrink(&mut self) {
        let target_length = self.parts.len() / 2;

        self.parts.truncate(target_length);
    }

    fn update_body(&mut self, old_head_x: f64, old_head_y: f64) {
        let (mut last_x, mut last_y) = (old_head_x, old_head_y);

        for i in 1..self.parts.len() {
            let mut part = &mut self.parts[i];
            let (tmp_x, tmp_y) = (last_x, last_y);
            last_x = part.position.x;
            last_y = part.position.y;
            part.position.x = tmp_x;
            part.position.y = tmp_y;
        }
    }
}

/**
* Struct holding details about one individual part of a snake
*/
#[derive(Debug)]
pub struct SnakePart {
    pub position: Position,
    // Position in the snake
    index: u32,
}

#[derive(Debug)]
pub enum MoveDirection {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
