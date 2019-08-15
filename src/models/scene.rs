use crate::models::game_state::GameState;
use crate::models::player::Player;
use piston::{*, UpdateArgs, Button::Keyboard};
use opengl_graphics::GlGraphics;

pub struct Scene {
    player: Player,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            player: Player::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, (w, h): (f64, f64)) {
        use graphics::*;

        const GREEN: [f32; 4] = [0., 1., 0., 0.7];

        gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
        });
        self.player.render(args, gl, (w, h));
    }

    pub fn key_event(&mut self, args: &Button, fps: usize) -> GameState {
        match args {
            Keyboard(k) => {
                match k {
                    Key::Escape => {
                         return GameState::Menu;
                    },
                    Key::Space => {
                         self.player.jump();
                    },
                    _ => {},
                }
                
            },
            _ => {},
        }
        GameState::None
    }

    pub fn update(&mut self, args: &UpdateArgs, (w, h): (f64, f64), fps: usize) {
        self.player.update(args, (w, h), fps);
    }
}