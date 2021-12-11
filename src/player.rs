use crate::world::World;
use glam::Vec2;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

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

        if is_key_pressed(KeyCode::Space) && self.velocity.y == 0.0 {
            self.velocity.y -= 30.0;
        }

        self.velocity.y += 0.8;

        world.camera.target = world.camera.target.lerp(
            (self.position.x + 8.0, self.position.y + 8.0).into(),
            3.0 * get_frame_time(),
        );

        // Apply the velocity
        self.position += self.velocity * get_frame_time();
        // Apply the friction
        self.velocity.x = self.velocity.x * ((1.0 - self.walking_friction) * self.walking_friction * get_frame_time());
        
        // Handle the collision
        for tilemap in &world.tilemaps {
            let min = ((self.position.x / tilemap.tile_size) as i32, -(self.position.y / tilemap.tile_size) as i32);
            let max = (((self.position.x + 16.0) / tilemap.tile_size) as i32, ((self.position.y + 16.0) / tilemap.tile_size) as i32);
            if tilemap.get(min).is_some() || tilemap.get(max).is_some() {
                self.position -= self.velocity * get_frame_time();
                self.velocity = Vec2::ZERO;
            }
        }
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
            walking_friction: 1.6,
        };
    }
}
