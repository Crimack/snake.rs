extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod app;
mod snake;

use app::App;
use snake::Snake;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_3;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("snake", [WIDTH, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl), Snake::new());

    let mut events = Events::new(EventSettings::new()).ups(30);
    while let Some(e) = events.next(&mut window)  {

        if !app.is_active() {
            println!("Cannabalism is bad! Game over!");
            break;
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(i) = e.button_args() {
            match (i.state, i.button) {
                (ButtonState::Press, Button::Keyboard(Key::W)) => {
                    app.move_up();
                }
                (ButtonState::Press, Button::Keyboard(Key::S)) => {
                    app.move_down();
                }
                (ButtonState::Press, Button::Keyboard(Key::A)) => {
                    app.move_left();
                }
                (ButtonState::Press, Button::Keyboard(Key::D)) => {
                    app.move_right();
                }
                (_, _) => {}
            }
        }
    }
}
