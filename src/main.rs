//! A 2d story-driven platformer with rpg-elements created by ComLarsic
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use futures::executor::block_on;
use macroquad::{
    hash,
    prelude::*,
    ui::{root_ui, widgets::Window},
};
use player::Player;
use world::{Tile, TileMap, World};

pub mod player;
pub mod world;

/** Handles the game struct */
pub struct Game {
    // The render target resolution
    pub target_resolution: (f32, f32),
    // The render target
    pub render_target: RenderTarget,
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
        // Create the render target
        let render_target = render_target(1280, 720);
        render_target.texture.set_filter(FilterMode::Nearest);
        
        return Self {
            target_resolution: (1280.0, 720.0),
            render_target,
            state: GameState::Startup,
            world: Rc::new(RefCell::new(World::new(render_target))),
        };
    }

    /** The games update loop */
    pub fn update(&mut self) {
        // Handle the game state
        match self.state {
            GameState::Startup => {
                // Set the state to gameplay
                self.state = GameState::TitleScreen;
            }
            GameState::Loading => {
                // Load the first scene
                let world = self.world.clone();
                let mut world = world.borrow_mut();
                block_on(world.load_level(world::Level {
                    player: Some(Player::default()),
                    tilemaps: vec![(
                        "assets/tilesets/grass_tiles.png".into(),
                        16.0,
                        vec![
                            vec![],
                            vec![
                                Some(Tile {
                                    location: (0, 0),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (1, 0),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (1, 0),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (2, 0),
                                    collision: true,
                                }),
                                None,
                                None,
                                None,
                                Some(Tile {
                                    location: (0, 0),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (2, 0),
                                    collision: true,
                                }),
                            ],
                            vec![
                                Some(Tile {
                                    location: (0, 1),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (1, 1),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (1, 1),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (2, 1),
                                    collision: true,
                                }),
                                None,
                                None,
                                None,
                                Some(Tile {
                                    location: (0, 2),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (2, 2),
                                    collision: true,
                                }),
                            ],
                            vec![
                                Some(Tile {
                                    location: (0, 2),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (1, 2),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (1, 2),
                                    collision: true,
                                }),
                                Some(Tile {
                                    location: (2, 2),
                                    collision: true,
                                }),
                                None,
                                None,
                                None,
                            ]
                        ],
                    )],
                }));
                // Set the gameplay state
                self.state = GameState::Gameplay;
            }
            GameState::TitleScreen => {
                if is_key_down(KeyCode::Enter) {
                    self.state = GameState::Loading;
                }
            }
            GameState::Gameplay => {
                // Update the world
                let world = self.world.clone();
                let mut world = world.borrow_mut();
                world.update(self);
            }
            GameState::Menu => {}
            GameState::Cutscene => {}
            GameState::Editor => {}
        }
    }

    /** Draw the update loop */
    pub fn draw(&mut self) {
        match self.state {
            GameState::Startup => {
                // Draw the startup screen
                clear_background(BLACK);
                draw_text("Startup", 0.0, 32.0, 32.0, WHITE);
            }
            GameState::Loading => {
                // Draw the loading screen
                clear_background(BLACK);
                draw_text("Loading", 0.0, 32.0, 32.0, WHITE);
            }
            GameState::TitleScreen => {
                set_camera(&Camera2D {
                    zoom: (0.001, -0.001 * screen_width() / screen_height()).into(),
                    ..Default::default()
                });
                clear_background(BLACK);
                draw_text("Highground", -575.0, -160.0, 256.0, WHITE);
                draw_text("-- Press Enter--", -275.0, 0.0, 64.0, WHITE);
            }
            GameState::Gameplay => {
                // Draw the world
                let world = self.world.clone();
                let world = world.borrow();
                world.draw(self);

                // Draw the worlds render texture
                set_default_camera();
                draw_texture_ex(self.render_target.texture, 0.0, 0.0, WHITE, DrawTextureParams {
                    dest_size: Some((screen_width(), screen_height()).into()),
                    source: Some(Rect {
                        x: 0.0,
                        y: 0.0,
                        w: self.target_resolution.0,
                        h: self.target_resolution.1,
                    }),
                    ..Default::default()
                });
            }
            GameState::Menu => {}
            GameState::Cutscene => {}
            GameState::Editor => {}
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
    };
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
