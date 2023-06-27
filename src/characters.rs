use macroquad::color::WHITE;
use macroquad::input::*;
use macroquad::prelude::Vec2;
use macroquad::texture::*;

pub struct Player
{
    pos: Vec2,
    speed: f32,
    texture: Texture2D
}

impl Player 
{
    pub fn new(speed: f32, texture: Texture2D) -> Self
    {
        let pos = Vec2::new(150.0, 150.0);

        Self { pos, speed, texture }
    }

    pub fn update(&mut self, dt: f32)
    {
        let x = match (is_key_down(KeyCode::A), is_key_down(KeyCode::D))
        {
            (true, false) => { -1f32 },
            (false, true) => { 1f32 },
            _ => 0f32
        };
        let y = match (is_key_down(KeyCode::W), is_key_down(KeyCode::S))
        {
            (true, false) => { -1f32 },
            (false, true) => { 1f32 },
            _ => 0f32
        };
        let input_vec = Vec2::new(x, y).normalize_or_zero();

        self.pos.x += input_vec.x * self.speed * dt;
        self.pos.y += input_vec.y * self.speed * dt;
    }

    pub fn render(&self)
    {
        let x = self.pos.x - (self.texture.width() / 2.0);
        let y = self.pos.y - (self.texture.height() / 2.0);
        draw_texture(self.texture, x, y, WHITE);
    }
}

