use macroquad::prelude::Texture2D;

/// constants for working with the tilemap
pub const DEFAULT_TILE_SIZE: u32 = 32;

/// describes the arrangement of sprites representing the world
pub struct Tilemap<'a>
{
    width: u32,
    height: u32,
    tile_size: u32,
    map: &'a [Texture2D]
}

impl<'a> Tilemap<'a>
{
    pub fn new(
        map: &'a mut [Texture2D],
        width: u32, height: u32, tile_size: u32
    ) -> Self
    {
        Self
        {
            width,
            height,
            tile_size,
            map
        }
    }
}

