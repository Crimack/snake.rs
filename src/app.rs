use piston::input::*;
use opengl_graphics::GlGraphics;
use graphics::*;

use snake::{Snake, SnakePart};
use snake::SNAKE_PART_WIDTH;
use snake::MoveDirection;

const BACKGROUND_WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SNAKE_BODY: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct App {
    // OpenGL drawing backend.
    gl: GlGraphics,
    snake: Snake,
}

impl App {
    pub fn new(gl: GlGraphics, snake: Snake) -> App {
        App { gl, snake }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let parts = &self.snake.parts;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND_WHITE, gl);

            let transform = c.transform;

            for part in parts {
                rectangle(
                    SNAKE_BODY,
                    rectangle::square(part.x, part.y, SNAKE_PART_WIDTH),
                    transform,
                    gl,
                );
            }
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
}
