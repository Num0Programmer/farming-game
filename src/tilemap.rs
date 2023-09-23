use macroquad::color::WHITE;
use macroquad::texture::*;


/// constants for working with the tilemap
pub const DEFAULT_TILE_SIZE: u32 = 32;


/// provides quick access to tile textures
pub type TileSet = Vec<Texture2D>;

/// describes the arrangement of sprites representing the world
pub struct TileMap
{
    width: u32,
    height: u32,
    tile_size: u32,
    tile_set: Option<TileSet>,
    map: Option<Vec<usize>>
}

impl TileMap
{
    pub fn new(
        width: u32,
        height: u32,
        tile_size: u32,
        tile_set: TileSet,
        map: Vec<usize>
    ) -> Self
    {
        Self
        {
            width,
            height,
            tile_size,
            tile_set: Some(tile_set),
            map: Some(map)
        }
    }

    pub fn render(&self)
    {
        // grab refs to tile set and map
        let map = self.map.as_ref().unwrap();
        let set = self.tile_set.as_ref().unwrap();

        let mut map_idx: usize;

        for row in 0..self.height
        {
            for col in 0..self.width
            {
                map_idx = (row * self.width + col) as usize;

                println!("map index = {}", map_idx);
                
                draw_texture(
                    set[map[map_idx]],
                    (self.tile_size * col) as f32,
                    (self.tile_size * row) as f32,
                    WHITE
                );
            }
        }
    }
}
