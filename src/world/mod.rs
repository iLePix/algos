extern crate perlin_noise as perlin;
use vecm::vec::*;
use perlin::PerlinNoise;

pub struct World {
    pub size: Vec2u,
    tiles: Vec<Tile>
}

impl World {
    pub fn new(size: Vec2u) -> Self {
        Self {size, tiles: vec![Tile::Land; (size.x * size.y) as usize]}
    }

    pub fn p(&mut self) {
        let perlin = PerlinNoise::new();
        let plus = 0.06;
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let height = perlin.get2d([x as f64 + 0.74, y as f64 + 0.74]);
                if height > 0.5 {
                    self.set_tile(Vec2u::new(x, y), Tile::Water);
                } else {
                    self.set_tile(Vec2u::new(x, y), Tile::Land);
                }
            }
        }
    }

    pub fn get_tile(&self, pos: Vec2u) -> Tile {
        assert!(pos.x < self.size.x && pos.y < self.size.y);
        self.tiles[(pos.x * self.size.y + pos.y) as usize]
    }

    pub fn set_tile(&mut self, pos: Vec2u, tile: Tile) {
        assert!(pos.x < self.size.x && pos.y < self.size.y);
        self.tiles[(pos.x * self.size.y + pos.y) as usize] = tile;
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Land,
    Water,
    Selected
}