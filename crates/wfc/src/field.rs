use std::collections::HashSet;

use crate::tileset::{WaveFunctionTileHandle, WaveFunctionTileset};
use macroquad::rand::RandomRange;
use utilities::infinite_grid::InfiniteGrid;

pub struct WaveFunctionCell {
    states: HashSet<WaveFunctionTileHandle>,
}

impl WaveFunctionCell {

    pub fn new_empty(tileset: &WaveFunctionTileset) -> Self {
        WaveFunctionCell {
            states: tileset.get_high_entropy_cache_clone()
        }
    }

    pub fn new_collapsed(state: WaveFunctionTileHandle) -> Self {
        let mut states = HashSet::new();
        states.insert(state);

        WaveFunctionCell {
            states
        }
    }

    #[inline]
    pub fn get_entropy(&self) -> usize {
        self.states.len()
    }

    pub fn collapse(&mut self, tileset: &WaveFunctionTileset) {
        let mut collapse_selector: Vec<(WaveFunctionTileHandle, f32)> = Vec::new();
        let mut running_weight: f32 = 0.0;

        // Select a random state and disregard the others, use the weights from the tileset to select
        for tile_handle in self.states.iter() {
            if let Some(class) = tileset.get_class_from_tile(tile_handle) {
                let weight = tileset.get_weight(&class);
                running_weight += weight;
                collapse_selector.push((*tile_handle, weight));
            }
        }

        let mut selected_weight = f32::gen_range(0.0, running_weight);
        let selected = collapse_selector.iter().find(|&data| {
            selected_weight -= data.1;
            selected_weight <= 0.0
        });

        if let Some(selected_data) = selected {
            self.states.clear();
            self.states.insert(selected_data.0);
        } else {
            // Some error state...
            panic!("Unable to collapse to a tile! Selected Weight: {:?}, States: {:?}", selected_weight, collapse_selector);
        }
    }

    pub fn get_tile_data(&self) -> Option<WaveFunctionTileHandle> {
        if self.get_entropy() == 1 {
            self.states.iter().next().cloned()
        } else {
            None
        }
    }
}

pub struct WaveFunctionSector {
    // width: usize,
    // height: usize,

    cells: Vec<WaveFunctionCell>,
}

impl WaveFunctionSector {
    fn new(tileset: &WaveFunctionTileset, width: usize, height: usize) -> Self {

        let cell_count = width * height;
        let mut cells = Vec::with_capacity(cell_count);

        for _ in 0..cell_count {
            cells.push(WaveFunctionCell::new_empty(tileset));
        }

        WaveFunctionSector {
            // width,
            // height,
            cells,
        }
    }
}



pub struct WaveFunctionField {
    sectors: InfiniteGrid<WaveFunctionSector>,
    sector_width: usize,
    sector_height: usize,

    tileset: WaveFunctionTileset,
}

impl WaveFunctionField {
    pub fn new(tileset: WaveFunctionTileset) -> Self {
        WaveFunctionField {
            sectors: InfiniteGrid::new(),
            sector_width: 16,
            sector_height: 16,
            tileset,
        }
    }

    pub fn collapse_sector(&mut self, x: i32, y: i32) {
        if let Some(sector) = self.sectors.get_mut(x, y) {
            // Do a trivial collapse just to test some stuff!
            for cell in sector.cells.iter_mut() {
                cell.collapse(&self.tileset)
            }
        }
    }

    pub fn add_sector(&mut self, x: i32, y: i32) {
        if let None = self.sectors.get(x, y) {
            self.sectors.set(x, y, WaveFunctionSector::new(&self.tileset, self.sector_width, self.sector_height));
        } else {
            // Some error condition!
            panic!("Attempting to add sector to occupied location ({:?},{:?})!", x, y);
        }
    }

    pub fn get_cell_render_data(&self, f: &dyn Fn((&str, f32))) {
        if let Some(sector) = self.sectors.get(0, 0) {
            for cell in sector.cells.iter() {
                if let Some(data) = cell.get_tile_data() {
                    if let Some(render_data) = self.tileset.get_render_data(&data) {
                        f(render_data);
                    }
                }
            }
        }
    }
}

