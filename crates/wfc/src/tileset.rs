use std::{collections::{HashMap, HashSet, hash_map::Entry}, hash::Hash};

use macroquad::prelude::load_string;
use nanoserde::DeJson;

#[derive(DeJson)]
struct TileData {
    id: String,
    texture_id: String,
    edge_ids: Vec<String>,
    can_rotate: bool,
}

#[derive(DeJson)]
struct Rule(String, String);

#[derive(DeJson)]
struct Weight(String, f32);

#[derive(DeJson)]
pub struct TilesetData {
    tiles: Vec<TileData>,
    rules: Vec<Rule>,
    weights: Vec<Weight>,
}

impl TilesetData {
    pub async fn from_data(data_path: &str) -> Result<Self, String> {
        if let Ok(contents) = load_string(data_path).await {
            if let Ok(tileset) = TilesetData::deserialize_json(&contents) {
                Ok(tileset)
            } else {
                Err(format!("Unable to parse input as TilesetData: {}", data_path))
            }
        } else {
            Err(format!("Unable to load TilesetData at path: {}", data_path))
        }
    }
}

pub const DIRECTION_UP: usize = 0;
pub const DIRECTION_RIGHT: usize = 1;
pub const DIRECTION_DOWN: usize = 2;
pub const DIRECTION_LEFT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaveFunctionTileClassHandle(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaveFunctionTileHandle(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaveFunctionEdgeHandle(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaveFunctionTextureHandle(usize);

#[derive(Debug, Clone, Copy)]
pub struct WaveFunctionRule(WaveFunctionEdgeHandle, WaveFunctionEdgeHandle);

#[derive(Debug, Clone, Copy)]
pub struct WaveFunctionWeight(WaveFunctionTileClassHandle, f32);

#[derive(Debug, Clone, Copy)]
pub struct WaveFunctionTile {
    edge_ids: [WaveFunctionEdgeHandle; 4],
    texture_id: WaveFunctionTextureHandle,
    rotation: u32,
    class_id: WaveFunctionTileClassHandle,
}

pub struct WaveFunctionTileset {
    tile_id_map: Vec<String>,
    edge_id_map: Vec<String>,
    texture_id_map: Vec<String>,

    tiles: Vec<WaveFunctionTile>,
    rules: Vec<WaveFunctionRule>,
    weights: Vec<WaveFunctionWeight>,

    high_entropy_cache: HashSet<WaveFunctionTileHandle>,
    validity_cache: [HashMap<WaveFunctionEdgeHandle, HashSet<WaveFunctionTileHandle>>; 4],
}

impl WaveFunctionTileset {
    pub fn new(tileset_data: TilesetData) -> Self {

        let mut tileset = WaveFunctionTileset {
            tile_id_map: Vec::new(),
            edge_id_map: Vec::new(),
            texture_id_map: Vec::new(),
            tiles: Vec::new(),
            rules: Vec::new(),
            weights: Vec::new(),
            high_entropy_cache: HashSet::new(),
            validity_cache: [
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new()
            ],
        };

        // Process the tile data and create any permutations required by rotations
        for tile_data in tileset_data.tiles.into_iter() {

            // Get or add the current tile ID
            let tile_id = if let Some(found) = tileset.tile_id_map.iter().position(|id| *id == tile_data.id) {
                found
            } else {
                let next_index = tileset.tile_id_map.len();
                tileset.tile_id_map.push(tile_data.id.clone());
                next_index
            };

            // Get or add the current texture ID
            let texture_id = if let Some(found) = tileset.texture_id_map.iter().position(|id| *id == tile_data.texture_id) {
                found
            } else {
                let next_index = tileset.texture_id_map.len();
                tileset.texture_id_map.push(tile_data.texture_id.clone());
                next_index
            };

            // Process the edge ids for the tile
            let mut edges: [WaveFunctionEdgeHandle; 4] = [WaveFunctionEdgeHandle(0); 4];
            for (index, edge_id) in tile_data.edge_ids.iter().enumerate() {
                let edge_index = if let Some(found) = tileset.edge_id_map.iter().position(|id| id == edge_id) {
                    found
                } else {
                    let next_index = tileset.edge_id_map.len();
                    tileset.edge_id_map.push(edge_id.to_owned());
                    next_index
                };

                edges[index] = WaveFunctionEdgeHandle(edge_index);
            }

            let wf_tile = WaveFunctionTile {
                edge_ids: edges,
                texture_id: WaveFunctionTextureHandle(texture_id),
                rotation: 0,
                class_id: WaveFunctionTileClassHandle(tile_id),
            };

            if tile_data.can_rotate {
                for r in 1..4 {
                    let mut tile = wf_tile.clone();
                    // Perform the rotation by shifting the edges to the right
                    // This is due to the fact that the edge representation processes clockwise starting with the top edge
                    tile.edge_ids.rotate_right(r);
                    tile.rotation = r as u32;

                    tileset.tiles.push(tile);
                }
            }

            tileset.tiles.push(wf_tile);
        }

        // Populate the high entropy cache to simplify the creation of high entropy cells
        for (handle, _) in tileset.tiles.iter().enumerate() {
            tileset.high_entropy_cache.insert(WaveFunctionTileHandle(handle));
        }

        // Process the rules
        for rule in tileset_data.rules.into_iter() {
            if let Some(leading_edge) = tileset.get_edge_handle(&rule.0) {
                if let Some(trailing_edge) = tileset.get_edge_handle(&rule.1) {
                    tileset.rules.push(WaveFunctionRule(leading_edge, trailing_edge));
                }
            }
        }

        // Process the weights
        for weight in tileset_data.weights.into_iter() {
            if let Some(class_handle) = tileset.get_tile_class_handle(&weight.0) {
                tileset.weights.push(WaveFunctionWeight(class_handle, weight.1));
            }
        }



        let mut cache: HashMap<WaveFunctionEdgeHandle, HashSet<WaveFunctionEdgeHandle>> = HashMap::new();
        let mut relevant_edges: HashSet<WaveFunctionEdgeHandle> = HashSet::new();

        // Cache valid edge pairings per edge direction based upon the rules
        for rule in tileset.rules.iter() {

            relevant_edges.insert(rule.0);
            relevant_edges.insert(rule.1);

            match cache.get_mut(&rule.0) {
                Some(valid_set) => {
                    valid_set.insert(rule.1);
                },
                None => {
                    let mut valid_set = HashSet::new();
                    valid_set.insert(rule.1);
                    cache.insert(rule.0, valid_set);
                }
            }

            match cache.get_mut(&rule.1) {
                Some(valid_set) => {
                    valid_set.insert(rule.0);
                },
                None => {
                    let mut valid_set = HashSet::new();
                    valid_set.insert(rule.0);
                    cache.insert(rule.1, valid_set);
                }
            }
        }

        for edge in relevant_edges.iter() {

            let valid_edges = cache.get(edge).unwrap();

            // Handle the UP case, match edge with any tiles that satisfy the edge rule with their DOWN
            {
                let tiles = tileset.validity_cache[DIRECTION_UP].entry(*edge).or_default();

                for (handle, tile) in tileset.tiles.iter().enumerate() {
                    if valid_edges.contains(&tile.edge_ids[DIRECTION_DOWN]) {
                        tiles.insert(WaveFunctionTileHandle(handle));
                    }
                }
            }

            // Handle the DOWN case, match edge with any tiles that satisfy the edge rule with their UP
            {
                let tiles = tileset.validity_cache[DIRECTION_DOWN].entry(*edge).or_default();

                for (handle, tile) in tileset.tiles.iter().enumerate() {
                    if valid_edges.contains(&tile.edge_ids[DIRECTION_UP]) {
                        tiles.insert(WaveFunctionTileHandle(handle));
                    }
                }
            }

            // Handle the RIGHT case, match edge with any tiles that satisfy the edge rule with their LEFT
            {
                let tiles = tileset.validity_cache[DIRECTION_RIGHT].entry(*edge).or_default();

                for (handle, tile) in tileset.tiles.iter().enumerate() {
                    if valid_edges.contains(&tile.edge_ids[DIRECTION_LEFT]) {
                        tiles.insert(WaveFunctionTileHandle(handle));
                    }
                }
            }

            // Handle the LEFT case, match edge with any tiles that satisfy the edge rule with their RIGHT
            {
                let tiles = tileset.validity_cache[DIRECTION_LEFT].entry(*edge).or_default();

                for (handle, tile) in tileset.tiles.iter().enumerate() {
                    if valid_edges.contains(&tile.edge_ids[DIRECTION_RIGHT]) {
                        tiles.insert(WaveFunctionTileHandle(handle));
                    }
                }
            }
        }

        tileset
    }

    pub fn get_edge_handle(&self, edge: &String) -> Option<WaveFunctionEdgeHandle> {
        if let Some(found) = self.edge_id_map.iter().position(|id| id == edge) {
            Some(WaveFunctionEdgeHandle(found))
        } else {
            None
        }
    }

    pub fn get_tile_class_handle(&self, tile_id: &String) -> Option<WaveFunctionTileClassHandle> {
        if let Some(found) = self.tile_id_map.iter().position(|id| id == tile_id) {
            Some(WaveFunctionTileClassHandle(found))
        } else {
            None
        }
    }

    pub fn get_high_entropy_cache_clone(&self) -> HashSet<WaveFunctionTileHandle> {
        self.high_entropy_cache.clone()
    }

    pub fn get_weight(&self, class_handle: &WaveFunctionTileClassHandle) -> f32 {
        match self.weights.iter().find(|&weight| weight.0 == *class_handle) {
            Some(found) => found.1,
            None => 0.0
        }
    }

    pub fn get_class_from_tile(&self, handle: &WaveFunctionTileHandle) -> Option<WaveFunctionTileClassHandle> {
        match self.tiles.get(handle.0) {
            Some(tile) => Some(tile.class_id),
            None => None,
        }
    }


}