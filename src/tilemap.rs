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

        for i in map
        {
            draw_texture(set[*i], 45.0, 45.0, WHITE);
        }
    }
}
