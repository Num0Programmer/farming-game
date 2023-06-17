use macroquad::color::WHITE;
use macroquad::input::*;
use macroquad::prelude::Vec2;
use macroquad::texture::*;

/// default speed for any character if a specific speed is not set
pub const DEFAULT_CHARACTER_SPEED: f32 = 400.0;

/// representation of a Character
pub struct Character
{
    pos: Vec2,
    speed: f32,
    texture: Texture2D
}

impl Character
{
    /// construct a new character
    pub fn new(speed: f32, texture: Texture2D) -> Self
    {
        let pos = Vec2::new(150.0, 150.0);

        Self { pos, speed, texture }
    }

    /// handles changing the character's state
    // TODO: bad implementaion if this is going to be a template for NPCs
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

    /// draws the character to the screen
    pub fn render(&self)
    {
        draw_texture(self.texture, self.pos.x, self.pos.y, WHITE);
    }
}

