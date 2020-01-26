use common::{Position, Positionable};
use snake::Snake;

pub enum EdibleType {
    FOOD,
    POISON,
}

pub struct Edible {
    pub position: Position,
    pub was_eaten: bool,
    pub edible_type: EdibleType,
}

impl Edible {
    pub fn new(x: f64, y: f64, edible_type: EdibleType) -> Edible {
        Edible {
            position: Position { x, y },
            was_eaten: false,
            edible_type: edible_type,
        }
    }

    pub fn eat(&mut self, snake: &mut Snake) {
        self.was_eaten = true;
        match self.edible_type {
            EdibleType::FOOD => snake.grow(),
            EdibleType::POISON => snake.shrink(),
        }
    }
}

impl Positionable for Edible {
    fn set_position(&mut self, x: f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
        self.was_eaten = false;
    }

    fn position(&self) -> &Position {
        &self.position
    }
}
