use macroquad::prelude::Texture2D;

/// constants for working with the tilemap
pub const DEFAULT_TILE_SIZE: u32 = 32;

/// describes the arrangement of sprites representing the world
pub struct Tilemap
{
    width: u32,
    height: u32,
    tile_size: u32,
    map: Option<Vec<usize>>
}

impl Tilemap
{
    pub fn new(
        width: u32,
        height: u32,
        tile_size: u32,
        map: Vec<usize>
    ) -> Self
    {
        Self
        {
            width,
            height,
            tile_size,
            map: Some(map)
        }
    }

    pub fn draw(&self)
    {
        println!("Tilemap drawn!");
    }
}
