use rand::Rng;

use macroquad::experimental::animation::*;
use macroquad::color::WHITE;
use macroquad::prelude::Rect;
use macroquad::prelude::Vec2;
use macroquad::input::*;
use macroquad::texture::*;

use crate::CropGrid;

const REACH: f32 = 50.0;
const GRAB_RAD: f32 = 0.1;

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
    flyaway: bool,
    speed: f32,
    pos: Vec2,
    target: Option<Vec2>,
    rect: Rect,
    texture: Texture2D,
    anim_dat: AnimatedSprite
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

        let anim_dat = AnimatedSprite::new(
            32,
            32,
            &[
                Animation
                {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 5,
                    fps: 6
                }
            ],
            true
        );

        Self
        {
            flyaway: false,
            speed,
            pos,
            target: None,
            rect,
            texture,
            anim_dat
        }
    }
    
    fn find_target(&self, crop_grid: &CropGrid) -> Option<Vec2>
    {
        let mut save = 0;
        let mut count = 0;
        let mut index: i32 = -1;
        let mut rng = rand::thread_rng();

        while count < 3 && index == -1
        {
            save = rng.gen_range(0..crop_grid.crops.len());
            if let Some(plant) = crop_grid.crops[save].plant
            {
                index = save as i32;
            };

            count += 1;
        }

        if index < 0
        {
            return None;
        }
        
        Some(crop_grid.crops[index as usize].pos)
    }

    fn fly(&mut self, dt: f32)
    {
        if let Some(target) = self.target
        {
            let curr_speed = (
                self.speed
                * (self.pos.distance(target)
                * GRAB_RAD)
            ).clamp(0.0, self.speed);
            let heading = -(self.pos - target).normalize();
            self.pos += heading * curr_speed * dt;
        }
    }

    pub fn get_rect(&self) -> Rect
    {
        Rect::new(self.rect.x, self.rect.y, self.rect.w, self.rect.h)
    }

    pub fn update(&mut self, dt: f32, crop_grid: &mut CropGrid)
    {
        // check crow has not targeted a plant
        if self.target.is_none()
        {
            self.target = self.find_target(crop_grid);
        }
        // otherwise, check crow needs to get closer to plant
        else if matches!(
            &self.target, Some(target)
            if self.pos.distance(*target) > GRAB_RAD
        )
        {
            self.fly(dt);
        }
        // otherwise, assume crow can grab plant from cell
        else if let Some(target) = self.target
        {
            crop_grid.steal_from_cell(target);
            self.target = None;
        }
    }

    pub fn render(&mut self)
    {
        let x = self.pos.x - (self.texture.width() / 2.0);
        let y = self.pos.y - (self.texture.height() / 2.0);
        draw_texture_ex(
            self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams
            {
                source: Some(self.anim_dat.frame().source_rect),
                ..Default::default()
            }
        );

        self.anim_dat.update();
    }
}

