use crate::{player::Player, Game};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

// Represents an entity
pub type Entity<T> = Rc<RefCell<T>>;

/** Represents the game world */
pub struct World {
    // The player in the world
    pub player: Option<Entity<Player>>,
    // The camera in the world
    pub camera: Camera2D,
    // The size of the tiles in the world
    pub tile_size: f32,
    // The tilemap
    pub tilemaps: Vec<TileMap>,
}

/** Represents a level */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    // The player in the level
    pub player: Option<Player>,
    // The tilemaps in the level
    pub tilemaps: Vec<(String, f32, Vec<Vec<Option<Tile>>>)>,
}

/** The tilemap in a level */
#[derive(Debug, Clone)]
pub struct TileMap {
    // The drawn tiles in the level
    pub tiles: Vec<Vec<Option<Tile>>>,
    // The tile size for the tileset
    pub tile_size: f32,
    // The tileset
    pub tileset: Texture2D,
}

impl TileMap {
    /** Get a tile of a position */
    pub fn get(&self, pos: (i32, i32)) -> Option<Tile> {
        if let Some(tile_y) = self.tiles.get(pos.1 as usize) {
            if let Some(tile) = tile_y.get(pos.0 as usize) {
                return tile.clone();
            }
        }
        return None;
    }
}

/** Represents a tile */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    // The tiles id in the tilemap
    pub location: (u32, u32),
    // Check to see if the tile should be colliding
    pub collision: bool,
}

impl World {
    pub fn new(render_target: RenderTarget) -> Self {
        return Self {
            player: None,
            camera: Camera2D {
                zoom: (0.01, 0.01 * screen_width() / screen_height()).into(),
                render_target: Some(render_target),
                ..Default::default()
            },
            tilemaps: vec![],
            tile_size: 0.0,
        };
    }

    /** Load a level */
    pub async fn load_level(&mut self, level: Level) {
        // Load the player
        if let Some(player) = level.player {
            self.player = Some(Rc::new(RefCell::new(player.clone())));
        }
        // Load the tiles
        self.tilemaps.clear();
        for tile in level.tilemaps {
            let tileset = load_texture(&tile.0).await.expect("Invalid tilemap path");
            tileset.set_filter(FilterMode::Nearest);
            self.tilemaps.push(TileMap {
                tiles: tile.2,
                tile_size: tile.1,
                tileset,
            });
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

        // Draw the player
        if let Some(player) = self.player.clone() {
            let player = player.borrow();
            // Draw the player
            player.draw();
        }

        // Draw the tiles
        for map in &self.tilemaps {
            for y in 0..map.tiles.len() {
                if let Some(tile_y) = map.tiles.get(y) {
                    for x in 0..map.tiles[y].len() {
                        if let Some(Some(tile)) = tile_y.get(x) {
                            draw_texture_ex(
                                map.tileset,
                                x as f32 * map.tile_size,
                                y as f32 * map.tile_size,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some((map.tile_size, map.tile_size).into()),
                                    source: Some(Rect {
                                        x: tile.location.0 as f32 * map.tile_size,
                                        y: tile.location.1 as f32 * map.tile_size,
                                        w: map.tile_size,
                                        h: map.tile_size,
                                    }),
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }
            }
        }
    }
}
