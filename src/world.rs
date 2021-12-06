use std::{cell::RefCell, rc::Rc};
use macroquad::prelude::*;
use serde::{Serialize, Deserialize};
use crate::{player::Player, Game};

// Represents an entity
pub type Entity<T> = Rc<RefCell<T>>;

/** Represents the game world */
pub struct World {
    // The player in the world
    pub player: Option<Entity<Player>>,
    // The camera in the world
    pub camera: Camera2D,
}

/** Represents a level */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub player: Option<Player>,
}

impl World {
    pub fn new() -> Self {
        return Self {
            player: None,
            camera: Camera2D {
                zoom: (0.0075, 0.0075 * screen_width() / screen_height()).into(),
                ..Default::default()
            },
        }    
    }

    /** Load a level */
    pub fn load_level(&mut self, level: Level) {
        // Load the player
        if let Some(player) = level.player {
            self.player = Some(Rc::new(RefCell::new(player.clone())));
        }
    }

    /** Update the world */
    pub fn update(&mut self, game: &mut Game) {
        // Update the player
        if let Some(player) = self.player.clone() {
            let mut player = player.borrow_mut();
            // Update the player
            player.update(self);
        }
    }

    /** Draw the world */
    pub fn draw(&self, game: &mut Game) {
        // Set the camera
        set_camera(&self.camera);
        // Clear the brackgroud
        clear_background(Color::from_rgba(32, 32, 32, 255));

        draw_rectangle(-16.0, -16.0, 16.0, 16.0, RED);
        draw_rectangle(-16.0, -32.0, 64.0, 16.0, BLUE);

        // Draw the player
        if let Some(player) = self.player.clone() {
            let player = player.borrow();
            // Draw the player
            player.draw();
        }
    }
}