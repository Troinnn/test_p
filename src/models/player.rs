use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use crate::models::game::G;

pub const SPEED_X_MAX: f64 = 9.8.;

pub struct Player {
    pos: (f64, f64),
    size: usize,
    mass: f64,
    speed_y: f64,
    speed_x: f64,
    jump_flag: bool,
    fall_flag: bool,
}

impl Player {
    pub fn new() -> Self {
        Player{
            pos: (10., 10.),
            size: 50,
            mass: 1.,
            speed_y: 0.,
            speed_x: 0.,
            jump_flag: false,
            fall_flag: true,
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
        if !self.jump_flag && !self.fall_flag {
            self.speed_y -= G * self.speed_y / fps as f64;
            self.pos.1 -= self.speed_y;
            if self.speed_y.round() == 0.0 {
                self.fall_flag = true;
                self.speed_y = 0.;
            }
        }

        if self.fall_flag {
            self.speed_y += G / fps as f64;
            self.pos.1 += self.speed_y;            
        }      

        if self.fall_flag && self.pos.1 > (h / 2.) - self.size as f64 / 2. {
            self.jump_flag = true;
            self.fall_flag = false;
            self.speed_y = 0.;
            self.pos.1 = (h / 2.) - self.size as f64 / 2.;
        } else {
            self.jump_flag = false;
        }

        self.pos.0 += self.speed_x / fps as f64;
    }

    pub fn jump(&mut self) {
        if self.jump_flag && !self.fall_flag {
            self.jump_flag = false;
            self.speed_y += G;
        }     
    }

    pub fn left(&mut self) {
        if self.speed_x > -SPEED_X_MAX {
            self.speed_x = -SPEED_X_MAX;
        } else {
            self.speed_x -= G;
        }
        
    }

    pub fn right(&mut self) {
        if self.speed_x < SPEED_X_MAX {
            self.speed_x = SPEED_X_MAX;
        } else {
            self.speed_x += G;
        }
        
    }
}