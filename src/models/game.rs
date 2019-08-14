use piston::window::WindowSettings;
use piston::Window;
use glutin_window::{GlutinWindow};
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use fps_counter::FPSCounter;
use piston::{Event, RenderArgs, RenderEvent, input::*};


use crate::models::menu::*;
use crate::models::game_state::*;

#[warn(dead_code)]
pub struct Game {
    pub gl: GlGraphics,
    pub window: GlutinWindow,
    pub menu: Menu,
    pub fps: FPSCounter,
    pub game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let window: GlutinWindow = WindowSettings::new("test_p", [800, 800])
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

        
        let font_size = 20.;
        let fps = self.fps.tick();
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let transform = c.transform.trans(2., 790.);
            text::Text::new_color(WHITE, font_size as u32).draw(
                    ("FPS: ".to_string() + &fps.to_string()).as_str(),
                    &mut glyph_cache,
                    &c.draw_state,
                    transform, 
                    gl
            ).unwrap();
        });      
        match self.game_state {
            GameState::Menu => {
                self.menu.render(args, &mut self.gl);
            },
            GameState::Play => {
            },
            GameState::Exit => {

            },
            GameState::None => {},
        }
    }

    pub fn key_event(&mut self, args: &Button) {
        match self.game_state {
            GameState::Menu => {
                match self.menu.key_event(args) {
                    GameState::Play => {self.game_state = GameState::Play;},
                    GameState::Menu => {self.game_state = GameState::Menu;},
                    GameState::Exit => {self.game_state = GameState::Exit;},
                    GameState::None => {},
                }
            },
            GameState::Play => {},
            GameState::Exit => {},
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
    }
}