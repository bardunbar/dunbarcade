mod player;

use arcade::{TileSet, Arcade};
use atlas::TextureAtlas;
use camera_layer::CameraLayer;
use player::Player;
use macroquad::{window::{Conf, next_frame, clear_background}, prelude::{is_quit_requested, set_default_camera, set_camera, is_key_down, KeyCode, Vec2, BLACK}};

const WIDTH: i32 = 640;
const HEIGHT: i32 = 360;

#[cfg(debug_assertions)]
const DEBUG_SCREEN_SCALE: i32 = 3;
#[cfg(not(debug_assertions))]
const DEBUG_SCREEN_SCALE: i32 = 1;

const WINDOW_WIDTH: i32 = WIDTH * DEBUG_SCREEN_SCALE;
const WINDOW_HEIGHT: i32 = HEIGHT * DEBUG_SCREEN_SCALE;



fn window_conf() -> Conf {
    Conf {
        window_title: "DUNBARCADE".to_string(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut camera_layer = CameraLayer::new(WIDTH as f32, HEIGHT as f32);

    let atlas = TextureAtlas::from_data("assets/atlas/arcade_basic.json", Some("assets/atlas/arcade_basic.png")).await.unwrap();

    let tile_set = TileSet::from_basic(atlas, Vec2::new(32.0, 32.0));

    let arcade = Arcade::new();

    camera_layer.translate(8.0 * 32.0, 8.0 * 32.0);

    let mut player = Player::new().await;

    player.position = Vec2::new(8.0 * 32.0, 8.0 * 32.0);

    loop {

        player.update();

        let player_screen_pos = camera_layer.world_to_screen(player.position);
        let safe_min = Vec2::new(192.0 / WIDTH as f32 * WINDOW_WIDTH as f32, 128.0 / HEIGHT as f32 * WINDOW_HEIGHT as f32);
        let safe_max = Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32) - safe_min;

        let player_min_delta = player_screen_pos - safe_min;
        let player_max_delta = player_screen_pos - safe_max;

        if player_min_delta.x < 0.0 || player_max_delta.x > 0.0 {
            camera_layer.translate(player.velocity.x, 0.);
        }
        if player_min_delta.y < 0.0 || player_max_delta.y > 0.0 {
            camera_layer.translate(0., player.velocity.y);
        }

        set_camera(&camera_layer.camera);

        clear_background(BLACK);

        for x in -1..=1 {
            for y in -1..=1 {
                if let Some(sector) = arcade.sectors.get(x, y) {
                    sector.draw(&tile_set);
                }
            }
        }

        player.draw();

        set_default_camera();

        camera_layer.draw();

        if is_quit_requested() || is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

mod arcade {

    use atlas::TextureAtlas;
    use macroquad::prelude::{Vec2, WHITE};

    use crate::infinite_grid::InfiniteGrid;

    const DEG_TO_RAD: f32 = 3.14159 / 180.;
    const ROT_90: f32 = 90. * DEG_TO_RAD;
    const ROT_180: f32 = 180. * DEG_TO_RAD;
    const ROT_270: f32 = 270. * DEG_TO_RAD;

    pub struct TileSet {
        tile_info: Vec<TileInfo>,
        atlas: TextureAtlas,
        tile_size: Vec2,
    }

    impl TileSet {
        pub fn from_basic(atlas: TextureAtlas, tile_size: Vec2) -> Self {
            let tile_vec = [
                // Tile Index 0 - Carpet
                TileInfo {
                    texture_id: "arcade_basic_carpet.png".to_owned(),
                    rotation: 0.
                },
                // Tile Index 1 - Floor Straight X
                TileInfo {
                    texture_id: "arcade_basic_floor_straight.png".to_owned(),
                    rotation: 0.
                },
                // Tile Index 2 - Floor Straight Y
                TileInfo {
                    texture_id: "arcade_basic_floor_straight.png".to_owned(),
                    rotation: ROT_90
                },
                // Tile Index 3 - Corner BR
                TileInfo {
                    texture_id: "arcade_basic_floor_corner.png".to_owned(),
                    rotation: 0.
                },
                // Tile Index 4 - Corner BL
                TileInfo {
                    texture_id: "arcade_basic_floor_corner.png".to_owned(),
                    rotation: ROT_90
                },
                // Tile Index 5 - Corner TL
                TileInfo {
                    texture_id: "arcade_basic_floor_corner.png".to_owned(),
                    rotation: ROT_180
                },
                // Tile Index 6 - Corner TR
                TileInfo {
                    texture_id: "arcade_basic_floor_corner.png".to_owned(),
                    rotation: ROT_270
                },
                // Tile Index 7 - TNT
                TileInfo {
                    texture_id: "arcade_basic_floor_t.png".to_owned(),
                    rotation: 0.
                },
                // Tile Index 8 - TNR
                TileInfo {
                    texture_id: "arcade_basic_floor_t.png".to_owned(),
                    rotation: ROT_90
                },
                // Tile Index 9 - TNB
                TileInfo {
                    texture_id: "arcade_basic_floor_t.png".to_owned(),
                    rotation: ROT_180
                },
                // Tile Index 10 - TNL
                TileInfo {
                    texture_id: "arcade_basic_floor_t.png".to_owned(),
                    rotation: ROT_270
                },
                // Tile Index 11 - Cross
                TileInfo {
                    texture_id: "arcade_basic_floor_cross.png".to_owned(),
                    rotation: 0.
                },
            ];

            TileSet {
                tile_info: tile_vec.to_vec(),
                atlas,
                tile_size,
            }
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct TileInfo {
        texture_id: String,
        rotation: f32,
    }

    pub struct ArcadeSector {
        width: u32,
        height: u32,
        position: Vec2,

        tiles: Vec<usize>,
    }

    impl ArcadeSector {

        pub fn from_indices(width: u32, height: u32, position: Vec2, indices: &[usize]) -> Self {
            ArcadeSector { width, height, position, tiles: indices.to_vec() }
        }

        pub fn draw(&self, tile_set: &TileSet) {

            for (tile_index, tile_info_handle) in self.tiles.iter().enumerate() {
                let (x, y) = (tile_index as u32 % self.width, tile_index as u32 / self.height);
                let (tile_x, tile_y) = ((x as f32 * tile_set.tile_size.x) + self.position.x, (y as f32 * tile_set.tile_size.y) + self.position.y);

                let info = &tile_set.tile_info[*tile_info_handle];
                tile_set.atlas.draw_texture(&info.texture_id, tile_x, tile_y, info.rotation, WHITE);
            }

        }
    }

    pub struct Arcade {
        pub sectors: InfiniteGrid<ArcadeSector>
    }

    impl Arcade {
        pub fn new() -> Self {

            let mut sectors = InfiniteGrid::new();
            sectors.set(0, 0, ArcadeSector::from_indices(16, 16, Vec2::new(0.0, 0.0), &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 1, 1, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 11, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            ]));
            sectors.set(1, 0, ArcadeSector::from_indices(16, 16, Vec2::new(16. * 32., 0.0), &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 1, 1, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 11, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            ]));
            sectors.set(-1, 0, ArcadeSector::from_indices(16, 16, Vec2::new(-16. * 32., 0.0), &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 1, 1, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 11, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            ]));
            sectors.set(0, 1, ArcadeSector::from_indices(16, 16, Vec2::new(0.0, 16. * 32.), &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 1, 1, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 11, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            ]));
            sectors.set(0, -1, ArcadeSector::from_indices(16, 16, Vec2::new(0.0, -16. * 32.), &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 1, 1, 1, 1, 1, 1, 4, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 3, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 11, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            ]));

            Arcade { sectors }
        }
    }

}

mod infinite_grid {
    use std::{collections::HashMap};


    pub struct InfiniteGrid<T> {
        map: HashMap<u64, T>,
    }

    impl<T> InfiniteGrid<T> {
        pub fn new() -> Self {
            InfiniteGrid::<T> {
                map: HashMap::new(),
            }
        }

        pub fn get(&self, x: i32, y: i32) -> Option<&T> {
            let hash = Self::to_hash(x, y);
            self.map.get(&hash)
        }

        pub fn set(&mut self, x: i32, y: i32, value: T) {
            let hash = Self::to_hash(x, y);
            self.map.insert(hash, value);
        }

        pub fn to_hash(x: i32, y:i32) -> u64 {
            let ux: u64 = x as u64;
            let uy: u64 = y as u64;
            let sy = uy << 32;
            ux.wrapping_add(sy)
        }

        pub fn from_hash(hash: u64) -> (i32, i32) {
            let ux = hash as u32;
            let uy = (hash >> 32) as u32;
            (ux as i32, uy as i32)
        }
    }

}