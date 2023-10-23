use macroquad::color::WHITE;
use macroquad::texture::*;


/// constants for working with the tilemap
pub const DEFAULT_TILE_SIZE: u32 = 32;


/// provides quick access to tile textures
pub type TileSet = Vec<Texture2D>;

/// describes the arrangement of sprites representing the world
pub struct TileMap
{
    /// extent of tile map on x-axis
    width: u32,
    /// extent of tile map on y-axis
    height: u32,
    /// dimension of tiles
    tile_size: u32,
    /// tiles which will be drawn to the screen
    tile_set: Option<TileSet>,
    /// mapping of tiles to positions in tile map
    map: Option<Vec<usize>>,
    /// pre-rendered map to speed up draw call
    i_map: Image
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
        let mut i_map = Image::gen_image_color(
            width as u16,
            height as u16,
            WHITE
        );
        let i_map_data = i_map.get_image_data_mut();

        let mut i_map_idx: usize;

        for row in 0..height
        {
            for col in 0..width
            {
                i_map_idx = (row * width + col) as usize;
                i_map_data[i_map_idx] = [255, 0, 0, 255];
            }
        }
        
        Self
        {
            width,
            height,
            tile_size,
            tile_set: Some(tile_set),
            map: Some(map),
            i_map
        }
    }

    pub fn render(&self)
    {
        draw_texture(
            Texture2D::from_image(&self.i_map),
            self.width as f32,
            self.height as f32,
            WHITE
        );
    }
}
