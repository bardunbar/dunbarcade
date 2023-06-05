use std::collections::HashMap;


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

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        let hash = Self::to_hash(x, y);
        self.map.get_mut(&hash)
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