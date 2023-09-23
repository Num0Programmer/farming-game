use macroquad::prelude::Texture2D;


/// constants for working with the tilemap
pub const DEFAULT_TILE_SIZE: u32 = 32;


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

    pub fn draw(&self)
    {
        println!("TileMap drawn!");
    }
}
