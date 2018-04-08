use common::Position;
use piston::input::*;
use opengl_graphics::GlGraphics;
use graphics::*;

use snake::Snake;
use snake::SNAKE_PART_WIDTH;
use snake::MoveDirection;
use edibles::Food;

// Color values
const BACKGROUND_WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SNAKE_BODY: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GOOD_EDIBLE: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BAD_EDIBLE: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App <'a> {
    // OpenGL drawing backend.
    gl: GlGraphics,
    snake: Snake,
    food: Vec<Food<'a>>
}

impl App {
    pub fn new(gl: GlGraphics, snake: Snake) -> App {
        App { gl, snake}
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let snake_body = &self.snake.parts;
        let good_edibles = &self.food;

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

            for part in snake_body {
                rectangle(
                    SNAKE_BODY,
                    rectangle::square(part.position.x, part.position.y, SNAKE_PART_WIDTH),
                    transform,
                    gl,
                );
            }

//            rectangle(
//                BAD_EDIBLE,
//                rectangle::square(
//                    self.poison.position.x,
//                    self.poison.position.y,
//                    SNAKE_PART_WIDTH,
//                ),
//                transform,
//                gl,
//            );
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.snake.update();
    }

    pub fn move_up(&mut self) {
        self.snake.current_direction = MoveDirection::UP;
    }

    pub fn move_down(&mut self) {
        self.snake.current_direction = MoveDirection::DOWN;
    }

    pub fn move_left(&mut self) {
        self.snake.current_direction = MoveDirection::LEFT;
    }

    pub fn move_right(&mut self) {
        self.snake.current_direction = MoveDirection::RIGHT;
    }

    pub fn is_active(&self) -> bool {
        self.snake.alive
    }

    // For spawning we want a list of locations where other things are,
    // so that there are't any instant collisions
    pub fn get_object_locations(&self) -> Vec<&Position> {
        self.snake
            .parts
            .iter()
            .map(|i| &i.position)
            .collect::<Vec<&Position>>()
    }
}
