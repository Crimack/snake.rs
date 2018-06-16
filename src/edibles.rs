use snake::Snake;
use common::{Position, Positionable};

pub trait Edible {
    fn eat(&mut self, snake: &mut Snake);
}

pub struct Food {
    pub position: Position,
    pub was_eaten: bool,
}

impl Food {
    pub fn new(x: f64, y: f64) -> Food {
        Food {
            position: Position { x, y },
            was_eaten: false,
        }
    }
}

impl Edible for Food {
    fn eat(&mut self, snake: &mut Snake) {
        self.was_eaten = true;
    }
}

impl Positionable for Food {
    fn set_position(&mut self, x: f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
        self.was_eaten = false;
    }
}

pub struct Poison {
    pub position: Position,
    pub was_eaten: bool,
}

impl Poison {
    pub fn new(x: f64, y: f64) -> Poison {
        Poison {
            position: Position { x, y },
            was_eaten: false,
        }
    }
}

impl Edible for Poison {
    fn eat(&mut self, snake: &mut Snake) {
        self.was_eaten = true;
    }
}

impl Positionable for Poison {
    fn set_position(&mut self, x: f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
        self.was_eaten = false;
    }
}
