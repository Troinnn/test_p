use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use crate::models::game::G;

pub struct Player {
    pos: (f64, f64),
    size: usize,
    mass: f64,
    speed: f64,
}

impl Player {
    pub fn new() -> Self {
        Player{
            pos: (10., 10.),
            size: 50,
            mass: 1.,
            speed: 0.
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, (w, h): (f64, f64)) {
        use graphics::*;

        const BLUE: [f32; 4] = [0., 0., 1., 1.];

        gl.draw(args.viewport(), |c, gl| {
            let square = rectangle::square(self.pos.0, self.pos.1, self.size as f64);
            let transform = c.transform.trans(self.pos.0, self.pos.1);
            rectangle(BLUE, square, transform, gl);
        });  
    }

    pub fn update(&mut self, args: &UpdateArgs, (w, h): (f64, f64), fps: usize) {
        if self.pos.1 < h / 2. - self.size as f64 {
            self.speed += G / fps as f64;
            if self.speed > h - self.pos.1 {
                self.speed = self.pos.1 - h;
            } else {
                self.pos.1 += self.speed;
            }
        } else {
            self.speed = 0.;
            self.pos.1 = h / 2. - self.size as f64 / 2.;
        }
    }

    pub fn jump(&mut self) {
        self.pos.1 -= self.size as f64 - 1.;
        self.speed += G  * -0.1;
    }
}