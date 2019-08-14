extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate fps_counter;
extern crate find_folder;

mod models;

use models::game::*;
use piston::event_loop::*;

fn main() {
    let mut app = Game::new();
    let mut events = Events::new(EventSettings::new());
    events.set_ups(60);
    events.set_max_fps(120);
    events.swap_buffers(true);
    while let Some(e) = events.next(&mut app.window) {
        app.event(e);
    }
}