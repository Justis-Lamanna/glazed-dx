mod data;

#[macro_use]
extern crate lazy_static;

use std::any::Any;
use game_loop::game_loop;

use game_loop::winit::event::{Event, WindowEvent};
use game_loop::winit::event_loop::EventLoop;
use game_loop::winit::window::{Window, WindowBuilder};
use crate::data::constants::Species;
use crate::data::core::Player;
use crate::data::pokemon::Pokemon;

fn main() {
    let player = Player::create_player("Milo");
    let mon = Pokemon::create_from_species_level(&player, Species::Quilava, 8);
    println!("{:#?}", mon);
    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();
    //
    // game_loop(event_loop, window, GlazedDX::default(), 240, 0.1, |g| {
    //     g.game.update();
    // }, |g| {
    //     g.game.render(&g.window);
    // }, |g, event| {
    //     g.game.window(event);
    // });
}

struct GlazedDX {
    scene: Box<dyn Scene>
}

impl GlazedDX {
    pub fn update(&mut self) {
        self.scene.on_update();
    }

    pub fn render(&self, window: &Window) {
        self.scene.on_render(window);
    }

    // A very simple handler that returns false when CloseRequested is detected.
    pub fn window(&self, event: Event<()>) -> bool {
        self.scene.on_window_event(event)
    }
}

impl Default for GlazedDX {
    fn default() -> Self {
        GlazedDX {
            scene: Box::new(EmptyScene)
        }
    }
}

trait Scene {
    fn on_update(&mut self);
    fn on_render(&self, window: &Window);
    fn on_window_event(&self, event: Event<()>) -> bool;
}

struct EmptyScene;
impl Scene for EmptyScene {
    fn on_update(&mut self) { }
    fn on_render(&self, window: &Window) { }
    fn on_window_event(&self, event: Event<()>) -> bool { true }
}