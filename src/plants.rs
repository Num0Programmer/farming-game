use macroquad::color::WHITE;
use macroquad::prelude::Vec2;
use macroquad::texture::*;
use macroquad::window::*;

const CROP_ROWS: i32 = 4;
const CROPS_PER_ROW: i32 = 5;

/// structure which represents places on the ground crops can be planted
// TODO: might need to do something about the planter not know which crop the
// player wants to remove
pub struct CropGrid
{
    offset: Vec2,
    screen_partition: Vec2,
    texture: Texture2D
}

impl CropGrid
{
    pub fn new(x_offset: f32, y_offset: f32, texture: Texture2D) -> Self
    {
        let offset = Vec2::new(x_offset, y_offset);
        let screen_partition = Vec2::new(
            screen_width() / CROPS_PER_ROW as f32,
            screen_height() / CROP_ROWS as f32
        );

        Self { offset, screen_partition, texture }
    }

    pub fn render(&self)
    {
        for row in 0..CROP_ROWS
        {
            for col in 0..CROPS_PER_ROW
            {
                draw_texture(
                    self.texture,
                    col as f32 * self.screen_partition.x + self.offset.x,
                    row as f32 * self.screen_partition.y + self.offset.y,
                    WHITE
                );
            }
        }
    }
}

