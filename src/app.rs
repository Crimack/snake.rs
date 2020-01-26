use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand::{thread_rng, Rng};

use edibles::{Edible, EdibleType};
use snake::{MoveDirection, Snake, SNAKE_PART_WIDTH};

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
    edibles: Vec<Edible>,
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl,
            snake: Snake::new(),
            edibles: vec![
                // Good tings
                Edible::new(1.0, 3.0, EdibleType::FOOD),
                Edible::new(1.0, 3.0, EdibleType::FOOD),
                Edible::new(12.0, 3.0, EdibleType::FOOD),
                Edible::new(421.0, 34.0, EdibleType::FOOD),
                Edible::new(61.0, 39.0, EdibleType::FOOD),
                Edible::new(14.0, 3.0, EdibleType::FOOD),
                Edible::new(1.0, 3.0, EdibleType::FOOD),
                Edible::new(91.0, 33.0, EdibleType::FOOD),
                Edible::new(731.0, 323.0, EdibleType::FOOD),
                Edible::new(330.0, 450.0, EdibleType::FOOD),
                Edible::new(500.0, 553.0, EdibleType::FOOD),
                // Bad tings
                Edible::new(100.0, 200.0, EdibleType::POISON),
                Edible::new(400.0, 500.0, EdibleType::POISON),
                Edible::new(500.0, 400.0, EdibleType::POISON),
                Edible::new(350.0, 250.0, EdibleType::POISON),
            ],
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let snake_body = &self.snake.parts;
        let edibles = &self.edibles;

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

            for e in edibles {
                match e.edible_type {
                    EdibleType::FOOD => {
                        rectangle(
                            GOOD_EDIBLE,
                            rectangle::square(e.position.x, e.position.y, SNAKE_PART_WIDTH),
                            transform,
                            gl,
                        );
                    }
                    EdibleType::POISON => {
                        rectangle(
                            BAD_EDIBLE,
                            rectangle::square(e.position.x, e.position.y, SNAKE_PART_WIDTH),
                            transform,
                            gl,
                        );
                    }
                }
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.snake.update(&mut self.edibles);

        let (x, y) = self.new_random_position();

        for e in &mut self.edibles {
            if e.was_eaten {
                e.set_position(x, y)
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

        let edibles = self
            .edibles
            .iter()
            .map(|i| &i.position)
            .collect::<Vec<&Position>>();

        locations.extend(edibles);

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
