use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::{thread_rng, Rng};

use edibles::{Food, Poison};
use snake::MoveDirection;
use snake::Snake;
use snake::SNAKE_PART_WIDTH;

use common::{Position, Positionable, WORLD_HEIGHT, WORLD_WIDTH};

// Color values
const BACKGROUND_WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SNAKE_BODY: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GOOD_EDIBLE: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BAD_EDIBLE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App {
    // OpenGL drawing backend.
    gl: GlGraphics,
    snake: Snake,
    food: Vec<Food>,
    poison: Vec<Poison>,
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl,
            snake: Snake::new(),
            food: vec![
                Food::new(1.0, 3.0),
                Food::new(1.0, 3.0),
                Food::new(12.0, 3.0),
                Food::new(421.0, 34.0),
                Food::new(61.0, 39.0),
                Food::new(14.0, 3.0),
                Food::new(1.0, 3.0),
                Food::new(91.0, 33.0),
                Food::new(731.0, 323.0),
                Food::new(330.0, 450.0),
                Food::new(500.0, 553.0),
            ],
            poison: vec![
                Poison::new(100.0, 200.0),
                Poison::new(400.0, 500.0),
                Poison::new(500.0, 400.0),
                Poison::new(350.0, 250.0),
            ],
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let snake_body = &self.snake.parts;
        let good_edibles = &self.food;
        let bad_edibles = &self.poison;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND_WHITE, gl);

            let transform = c.transform;

            for part in snake_body {
                rectangle(
                    SNAKE_BODY,
                    rectangle::square(part.position.x, part.position.y, SNAKE_PART_WIDTH),
                    transform,
                    gl,
                );
            }

            for f in good_edibles {
                rectangle(
                    GOOD_EDIBLE,
                    rectangle::square(f.position.x, f.position.y, SNAKE_PART_WIDTH),
                    transform,
                    gl,
                );
            }

            for p in bad_edibles {
                rectangle(
                    BAD_EDIBLE,
                    rectangle::square(p.position.x, p.position.y, SNAKE_PART_WIDTH),
                    transform,
                    gl,
                );
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.snake.update(&mut self.food, &mut self.poison);

        let (x, y) = self.new_random_position();

        for mut f in &mut self.food {
            if f.was_eaten {
                f.set_position(x, y)
            }
        }

        for mut p in &mut self.poison {
            if p.was_eaten {
                p.set_position(x, y)
            }
        }
    }

    pub fn move_up(&mut self) {
        match self.snake.current_direction {
            MoveDirection::DOWN => {}
            _ => self.snake.current_direction = MoveDirection::UP,
        };
    }

    pub fn move_down(&mut self) {
        match self.snake.current_direction {
            MoveDirection::UP => {}
            _ => self.snake.current_direction = MoveDirection::DOWN,
        };
    }

    pub fn move_left(&mut self) {
        match self.snake.current_direction {
            MoveDirection::RIGHT => {}
            _ => self.snake.current_direction = MoveDirection::LEFT,
        };
    }

    pub fn move_right(&mut self) {
        match self.snake.current_direction {
            MoveDirection::LEFT => {}
            _ => self.snake.current_direction = MoveDirection::RIGHT,
        };
    }

    pub fn is_active(&self) -> bool {
        self.snake.alive
    }

    // For spawning we want a list of locations where other things are,
    // so that there are't any instant collisions
    fn get_object_locations(&self) -> Vec<&Position> {
        let mut locations = self
            .snake
            .parts
            .iter()
            .map(|i| &i.position)
            .collect::<Vec<&Position>>();

        let food = self
            .food
            .iter()
            .map(|i| &i.position)
            .collect::<Vec<&Position>>();

        let poison = self
            .poison
            .iter()
            .map(|i| &i.position)
            .collect::<Vec<&Position>>();

        locations.extend(food);
        locations.extend(poison);

        locations
    }

    fn new_random_position(&self) -> (f64, f64) {
        let locations = self.get_object_locations();
        let mut rng = thread_rng();

        loop {
            // Generate as u32 so we're guaranteed integer locations
            let x = rng.gen_range(0, WORLD_WIDTH as u32) as f64;
            let y = rng.gen_range(0, WORLD_HEIGHT as u32) as f64;
            if !locations.iter().any(|i| i.x == x && i.y == y) {
                return (x, y);
            }
        }
    }
}
