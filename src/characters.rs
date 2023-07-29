use rand::Rng;

use macroquad::color::WHITE;
use macroquad::prelude::Rect;
use macroquad::prelude::Vec2;
use macroquad::input::*;
use macroquad::texture::*;

use crate::CropGridCell;

const REACH: f32 = 50.0;

/// describes a player's character
pub struct Player
{
    speed: f32,
    pos: Vec2,
    rect: Rect,
    texture: Texture2D
}

/// functions specific to player controlled characters
impl Player
{
    pub fn new(speed: f32, texture: Texture2D) -> Self
    {
        let pos = Vec2::new(150.0, 150.0);
        let rect = Rect::new(pos.x, pos.y, REACH, REACH);

        Self { speed, pos, rect, texture }
    }

    pub fn get_rect(&self) -> Rect
    {
        Rect::new(self.rect.x, self.rect.y, self.rect.w, self.rect.h)
    }

    pub fn render(&self)
    {
        let x = self.pos.x - (self.texture.width() / 2.0);
        let y = self.pos.y - (self.texture.height() / 2.0);
        draw_texture(self.texture, x, y, WHITE);
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

/// describes a crow character
pub struct Crow
{
    speed: f32,
    pos: Vec2,
    target: Option<Vec2>,
    rect: Rect,
    texture: Texture2D
}

/// functions specific to crow character
impl Crow
{
    pub fn new(speed: f32, texture: Texture2D) -> Self
    {
        let pos = Vec2::new(
            macroquad::window::screen_width() / 2.0,
            macroquad::window::screen_height() / 2.0
        );
        let rect = Rect::default();

        Self
        {
            speed,
            pos,
            target: None,
            rect,
            texture
        }
    }
    
    fn find_target(&self, crops: &Vec<CropGridCell>) -> Option<Vec2>
    {
        let mut save = 0;
        let mut count = 0;
        let mut index: i32 = -1;
        let mut rng = rand::thread_rng();

        while count < 3 && index == -1
        {
            save = rng.gen_range(0..crops.len());
            if let Some(plant) = &crops[save].plant
            {
                index = save as i32;
            };

            count += 1;
        }

        if index < 0
        {
            None
        }
        else
        {
            Some(crops[index as usize].pos)
        }
    }

    fn fly(&mut self, dt: f32)
    {
        if let Some(t) = self.target
        {
            let heading = -(self.pos - t).normalize();
            self.pos += heading * self.speed * dt;
        };
    }

    pub fn get_rect(&self) -> Rect
    {
        Rect::new(self.rect.x, self.rect.y, self.rect.w, self.rect.h)
    }

    pub fn update(&mut self, dt: f32, crops: &Vec<CropGridCell>)
    {
        // check already targeted a plant
        // TODO: implement way for crow to be scared away and interest timer
        if matches!(
            &self.target,
            Some(target)
            if (self.pos.x - target.x).abs() > 2.0 && (self.pos.y - target.y).abs() > 2.0
        )
        {
            println!("Flying to {:?}", self.target);
            self.fly(dt);
        }
        // otherwise, assume need to find a new plant
        else
        {
            self.target = self.find_target(crops);
            println!("Chose plant at {:?}", self.target);
        }
    }

    pub fn render(&self)
    {
        let x = self.pos.x - (self.texture.width() / 2.0);
        let y = self.pos.y - (self.texture.height() / 2.0);
        draw_texture(self.texture, x, y, WHITE);
    }
}

