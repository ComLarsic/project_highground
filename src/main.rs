//! A 2d story-driven platformer with rpg-elements created by ComLarsic
use std::{cell::{RefCell, RefMut}, rc::Rc};

use macroquad::{prelude::*, ui::{root_ui, widgets::Window}, hash};
use player::Player;
use world::World;

pub mod world;
pub mod player;

/** Handles the game struct */
pub struct Game {
    // The game state
    pub state: GameState, 
    // The game world
    world: Rc<RefCell<World>>,
}

/** Represents the game state */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Startup,
    Loading,
    TitleScreen,
    Gameplay,
    Menu,
    Cutscene,
    // Only used in debug mode
    Editor,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            state: GameState::Startup,
            world: Rc::new(RefCell::new(World::new())),
        };
    }

    /** The games update loop */
    pub fn update(&mut self) {
        // Handle the game state
        match self.state {
            GameState::Startup => {
                // Set the state to gameplay
                self.state = GameState::TitleScreen;
            },
            GameState::Loading => {
                // Load the first scene
                let world = self.world.clone();
                let mut world = world.borrow_mut();
                world.load_level(world::Level { 
                    player: Some(Player::default()),
                });
                // Set the gameplay state
                self.state = GameState::Gameplay;
            },
            GameState::TitleScreen => {
                if is_key_down(KeyCode::Enter) {
                    self.state = GameState::Loading;
                }
            },
            GameState::Gameplay => {
                // Update the world
                let world = self.world.clone();
                let mut world = world.borrow_mut();
                world.update(self);
            },
            GameState::Menu => {},
            GameState::Cutscene => {},
            GameState::Editor => {},
        }
    }

    /** Draw the update loop */
    pub fn draw(&mut self) {
        match self.state {
            GameState::Startup => {
                // Draw the startup screen
                clear_background(BLACK);
                draw_text("Startup", 0.0, 32.0, 32.0, WHITE);
            },
            GameState::Loading => {
                // Draw the loading screen
                clear_background(BLACK);
                draw_text("Loading", 0.0, 32.0, 32.0, WHITE);
            },
            GameState::TitleScreen => {
                set_camera(&Camera2D{
                    zoom: (0.001, -0.001 * screen_width() / screen_height()).into(),
                    ..Default::default()
                });
                clear_background(BLACK);
                draw_text("Highground", -575.0, -160.0, 256.0, WHITE);
                draw_text("-- Press Enter--", -275.0, 0.0, 64.0, WHITE);
            },
            GameState::Gameplay => {
                // Draw the world
                let world = self.world.clone();
                let world = world.borrow();
                world.draw(self);
            },
            GameState::Menu => {},
            GameState::Cutscene => {},
            GameState::Editor => {},
        }
    }

    /** Return the world */
    pub fn world(&self) -> RefMut<World> {
        return self.world.borrow_mut();
    }
}

/** Return the window config */
fn conf() -> Conf {
    return Conf {
        window_title: "Highground".into(),
        window_width: 1280,
        window_height: 720,
        fullscreen: true,
        // Disable anti-aliasing
        sample_count: 0,
        ..Default::default()
    }
}

/** The entrypoint, just a standard macroquad entrypoint */
#[macroquad::main(conf)]
async fn main() {
    // Create the world
    let mut game = Game::new();

    // The gameloop
    loop {
        game.update();
        game.draw();
        //set_default_camera();
        next_frame().await;
    }
}
