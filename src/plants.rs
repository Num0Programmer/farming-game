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
    pos: Vec2,
    screen_partition: Vec2,
    texture: Texture2D
}

impl CropGrid
{
    pub fn new(x: f32, y: f32, texture: Texture2D) -> Self
    {
        let pos = Vec2::new(x, y);
        let screen_partition = Vec2::new(
            screen_width() / CROPS_PER_ROW as f32,
            screen_height() / CROP_ROWS as f32
        );
        Self { pos, screen_partition, texture }
    }

    pub fn render(&self)
    {
        let x_init = (self.pos.x / CROPS_PER_ROW as f32)
            - (self.texture.width() / 2.0) - 2.0;
        let mut y = (self.pos.y / CROP_ROWS as f32)
            - (self.texture.height() / 2.0);

        for _row in 0..CROP_ROWS
        {
            // reset x coordinate of next column
            let mut x = x_init;
            for _col in 0..CROPS_PER_ROW
            {
                draw_texture(self.texture, x, y, WHITE);
                x += self.screen_partition.x;
            }
            y += self.screen_partition.y;
        }
    }
}

