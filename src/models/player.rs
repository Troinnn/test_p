use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use crate::models::game::G;

pub struct Player {
    pos: (f64, f64),
    size: usize,
    mass: f64,
    speed: f64,
    jump_flag: bool,
    fall_flag: bool,
}

impl Player {
    pub fn new() -> Self {
        Player{
            pos: (10., 10.),
            size: 50,
            mass: 1.,
            speed: 0.,
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
            println!("{:?} : {:?} : {:?}", G, self.speed, fps as f64);
            self.speed -= G * self.speed / fps as f64;
            println!("{:?}", self.speed);
            self.pos.1 -= self.speed;
            if self.speed.round() == 0.0 {
                self.fall_flag = true;
                self.speed = 0.;
            }
        }

        if self.fall_flag {
            self.speed += G / fps as f64;
            if self.speed >= h - self.pos.1 {
                self.pos.1 = (h - self.size as f64) / 2.;
            } else {
                self.pos.1 += self.speed;
            }            
        }
        
        if self.pos.1 >= (h - self.size as f64) / 2. {
            self.fall_flag = false;
            self.jump_flag = true;
        } else {
            //self.fall_flag = true;
            //self.jump_flag = false;
        }
    }

    pub fn jump(&mut self) {
        if self.jump_flag && !self.fall_flag {
            self.jump_flag = false;
        }     
    }
}