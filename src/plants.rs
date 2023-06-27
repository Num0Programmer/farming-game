use macroquad::color::WHITE;
use macroquad::prelude::Vec2;
use macroquad::texture::*;

/// structure which represents places on the ground crops can be planted
pub struct CropRow
{
    pos: Vec2,
    texture: Texture2D
}

impl CropRow
{
    pub fn new(x: f32, y: f32, texture: Texture2D) -> Self
    {
        let pos = Vec2::new(x, y);

        Self { pos, texture }
    }

    pub fn render(&self)
    {
        // TODO: update so this renders all the crops first, then the dirt
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}

