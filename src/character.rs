use macroquad::color::WHITE;
use macroquad::input::*;
use macroquad::prelude::Vec2;
use macroquad::texture::*;

const DEFAULT_CHARACTER_SPEED: f32 = 400.0;

pub struct Character
{
    texture: Texture2D,
    pos: Vec2
}

impl Character
{
    pub fn new(texture: Texture2D) -> Self
    {
        let pos = Vec2::new(150.0, 150.0);

        Self { texture, pos }
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

        self.pos.x += input_vec.x * DEFAULT_CHARACTER_SPEED * dt;
        self.pos.y += input_vec.y * DEFAULT_CHARACTER_SPEED * dt;
    }

    pub fn render(&self)
    {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}

