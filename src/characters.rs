use std::marker::PhantomData;

use macroquad::color::WHITE;
use macroquad::prelude::Rect;
use macroquad::prelude::Vec2;
use macroquad::shapes::draw_rectangle;
use macroquad::input::*;
use macroquad::texture::*;

const REACH: f32 = 50.0;

// character classification
pub struct Player;
pub struct Crow;

pub struct Character<Class>
{
    pos: Vec2,
    rect: Rect,
    speed: f32,
    texture: Texture2D,
    class: PhantomData<Class>
}

impl Character<Player>
{
    pub fn new(speed: f32, texture: Texture2D) -> Self
    {
        let pos = Vec2::new(150.0, 150.0);
        let rect = Rect::new(pos.x, pos.y, REACH, REACH);
        let class = PhantomData::<Player>;

        Self { pos, rect, speed, texture, class }
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

        self.rect.x = self.pos.x - (REACH / 2.0);
        self.rect.y = self.pos.y - (REACH / 2.0);
    }
}

impl Character<Crow>
{
    pub fn new(speed: f32, texture: Texture2D) -> Self
    {
        let pos = Vec2::new(
            macroquad::window::screen_width() / 2.0,
            macroquad::window::screen_height() / 2.0
        );
        let rect = Rect::default();
        let class = PhantomData::<Crow>;

        Self { pos, rect, speed, texture, class }
    }
}

impl<Class> Character<Class>
{
    pub fn render(&self)
    {
        let x = self.pos.x - (self.texture.width() / 2.0);
        let y = self.pos.y - (self.texture.height() / 2.0);
        draw_texture(self.texture, x, y, WHITE);
    }

    pub fn get_rect(&self) -> Rect
    {
        Rect::new(self.rect.x, self.rect.y, self.rect.w, self.rect.h)
    }
}

