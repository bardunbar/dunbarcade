use atlas::TextureAtlas;
use macroquad::prelude::{Vec2, WHITE};

use utilities::infinite_grid::InfiniteGrid;

const DEG_TO_RAD: f32 = 3.14159 / 180.;
const ROT_90: f32 = 90. * DEG_TO_RAD;
const ROT_180: f32 = 180. * DEG_TO_RAD;
const ROT_270: f32 = 270. * DEG_TO_RAD;

pub struct TileSet {
    tile_info: Vec<TileInfo>,
    pub atlas: TextureAtlas,
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