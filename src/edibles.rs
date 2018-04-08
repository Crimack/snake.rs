use rand::{thread_rng, Rng};

use snake::Snake;
use common::{Position, WORLD_HEIGHT, WORLD_WIDTH};
use app::App;

pub trait Edible {
    fn eat(&mut self, snake: &mut Snake);
}

pub struct Food<'a> {
    position: Position,
    app: &'a App<'a>
}

impl<'a> Food<'a> {
    pub fn new(app: &'a App) -> Food {
        let (x, y) = new_random_position(app);
        Food {
            position: Position { x, y },
            app,
        }
    }
}

impl<'a> Edible for Food<'a> {
    fn eat(&mut self, snake: &mut Snake) {}
}

pub struct Poison<'a> {
    position: Position,
    app: &'a App<'a>,
}

impl<'a> Edible for Poison<'a> {
    fn eat(&mut self, snake: &mut Snake) {}
}

fn new_random_position(app: &App) -> (f64, f64) {
    let locations = app.get_object_locations();
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
