use piston::window::WindowSettings;
use piston::Window;
use glutin_window::{GlutinWindow};
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use fps_counter::FPSCounter;
use piston::{Event, RenderArgs, RenderEvent, input::*};


use crate::models::menu::*;
use crate::models::game_state::*;
use crate::models::scene::*;

pub const G: f64 = 9.81;

#[warn(dead_code)]
pub struct Game {
    pub gl: GlGraphics,
    pub window: GlutinWindow,
    pub menu: Menu,
    pub fps: FPSCounter,
    pub game_state: GameState,
    pub scene: Scene,
}

impl Game {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new("test_p", [600, 600])
            .graphics_api(opengl)
            .exit_on_esc(false)
            .build()
            .unwrap();

        Game {
            gl: GlGraphics::new(opengl),
            window,
            menu: Menu::new(),
            fps: FPSCounter::new(),
            game_state: GameState::Menu,
            scene: Scene::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0., 0., 0., 1.];
        const WHITE: [f32; 4] = [1., 1., 1., 1.];

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font = assets.join("FiraSans-Regular.ttf");
        let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

        let width = self.window.size().width;
        let height = self.window.size().height;

        let font_size = 20.;
        let fps = self.fps.tick();
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
        });      
        match self.game_state {
            GameState::Menu => {
                self.menu.render(args, &mut self.gl, (width, height));
            },
            GameState::Play => {
                self.scene.render(args, &mut self.gl, (width, height));
            },
            GameState::Exit => {

            },
            GameState::None => {},
        }

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(2., height - 10.);
            text::Text::new_color(WHITE, font_size as u32).draw(
                    ("FPS: ".to_string() + &fps.to_string()).as_str(),
                    &mut glyph_cache,
                    &c.draw_state,
                    transform, 
                    gl
            ).unwrap();
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let fps = self.fps.tick();
        let width = self.window.size().width;
        let height = self.window.size().height;
        match self.game_state {
            GameState::Menu => {},
            GameState::Play => {
                self.scene.update(args, (width, height), fps);
            },
            GameState::Exit => {},
            GameState::None => {},
        }
    }

    pub fn key_event(&mut self, args: &Button) {
        let fps = self.fps.tick();
        match self.game_state {
            GameState::Menu => {
                let new_state = self.menu.key_event(args);
                self.set_state(&new_state); 
            },
            GameState::Play => {
                let new_state = self.scene.key_event(args, fps);
                self.set_state(&new_state);
            },
            GameState::Exit => {},
            GameState::None => {},
        }
    }

    pub fn set_state(&mut self, new_state: &GameState) {
        match new_state {
            GameState::Play => {self.game_state = GameState::Play;},
            GameState::Menu => {self.game_state = GameState::Menu;},
            GameState::Exit => {self.game_state = GameState::Exit;},
            GameState::None => {},
        }
    }

    pub fn event(&mut self, event: Event) {
        if self.game_state == GameState::Exit {
            self.window.set_should_close(true);
        }
        
        if let Some(r) = event.render_args() {
            self.render(&r);
        }

        if let Some(k) = event.press_args() {
            self.key_event(&k);
        }

        if let Some(u) = event.update_args() {
            self.update(&u);
        }
    }
}