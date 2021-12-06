use macroquad::prelude::*;
use glam::Vec2;
use serde::{Serialize, Deserialize};
use crate::world::World;

/** Represents the player */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    // The players transformation in the world
    pub position: Vec2,
    pub scale: Vec2,

    pub velocity: Vec2,

    // The player stats
    pub walking_friction: f32,
}

impl Player {
    pub fn new() -> Self {
        return Self::default();
    }

    // Update the playerl
    pub fn update(&mut self, world: &mut World) {
        if is_key_down(KeyCode::D) {
            self.velocity.x += 100.0;
        }
        if is_key_down(KeyCode::A) {
            self.velocity.x -= 100.0;
        }
        
        world.camera.target = world.camera.target.lerp((self.position.x + 8.0, self.position.y + 8.0).into(), 3.0 * get_frame_time());

        // Apply the velocity
        self.position += self.velocity * get_frame_time();
        // Apply the friction
        self.velocity.x = self.velocity.x * (1.0 - self.walking_friction) * self.walking_friction;
    }

    /** Draw the player */
    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, 16.0, 16.0, LIME);
    }
}

impl Default for Player {
    fn default() -> Self {
        return Self {
            position: (0.0, 0.0).into(),
            scale: (1.0, 1.0).into(),
            velocity: (0.0, 0.0).into(),
            walking_friction: 0.16,
        }
    }
}