use macroquad::color;
use macroquad::experimental::animation::*;
use macroquad::input::*;
use macroquad::prelude::Rect;
use macroquad::prelude::screen_width;
use macroquad::prelude::screen_height;
use macroquad::prelude::Vec2;
use macroquad::rand::gen_range;
use macroquad::texture::*;

use crate::CropGrid;

pub const SCREEN_BORDER_EXT: f32 = 25.0;

const SPRITE_DIM_32: f32 = 32.0;

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
        draw_texture(self.texture, x, y, color::WHITE);
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
    vel: Vec2,
    pos: Vec2,
    target: Vec2,
    rect: Rect,
    texture: Texture2D,
    anim_dat: AnimatedSprite
}

/// functions specific to crow character
impl Crow
{
    pub fn new(speed: f32, pos: Vec2, texture: Texture2D) -> Self
    {
        let rect = Rect::new(pos.x, pos.y, SPRITE_DIM_32, SPRITE_DIM_32);
        let anim_dat = AnimatedSprite::new(
            32,
            32,
            &[
                Animation
                {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 4,
                    fps: 4
                }
            ],
            true
        );

        Self
        {
            flyaway: false,
            speed,
            vel: Vec2::ZERO,
            pos,
            target: Vec2::NAN,
            rect,
            texture,
            anim_dat
        }
    }
    
    fn find_target(&self, crop_grid: &CropGrid) -> Vec2
    {
        let mut save = 0;
        let mut count = 0;
        let mut index: i32 = -1;

        while count < 3 && index == -1
        {
            save = gen_range(0, crop_grid.crops.len());
            if let Some(plant) = crop_grid.crops[save].plant
            {
                index = save as i32;
            };

            count += 1;
        }

        if index < 0
        {
            return Vec2::NAN;
        }
        
        crop_grid.crops[index as usize].pos
    }

    fn fly(&mut self, dt: f32)
    {
        if (-SCREEN_BORDER_EXT <= self.pos.x
                && self.pos.x <= screen_width() + SCREEN_BORDER_EXT)
            && (-SCREEN_BORDER_EXT <= self.pos.y
                  && self.pos.y <= screen_height() + SCREEN_BORDER_EXT)
        {
            self.pos += self.vel * dt;
        }
        else
        {
            self.pos.x = self.pos.x.clamp(
                -SCREEN_BORDER_EXT + 1.0,
                screen_width() + (SCREEN_BORDER_EXT - 1.0)
            );
            self.pos.y = self.pos.y.clamp(
                -SCREEN_BORDER_EXT + 1.0,
                screen_height() + (SCREEN_BORDER_EXT - 1.0)
            );

            self.flyaway = false;
            self.vel = Vec2::ZERO;
        }
    }

    pub fn get_rect(&self) -> Rect
    {
        Rect::new(self.rect.x, self.rect.y, self.rect.w, self.rect.h)
    }

    pub fn update(&mut self, dt: f32, crop_grid: &mut CropGrid)
    {
        // check crow stole a crop
        if self.flyaway
        {
            self.vel += 6.0; // scary magic number
        }
        // otherwise, check crow has not targeted a plant
        else if self.target.is_nan()
        {
            self.target = self.find_target(crop_grid);
        }
        // otherwise, check close enough to steal plant
        else if self.pos.distance(self.target) <= GRAB_RAD
        {
            crop_grid.steal_from_cell(self.target);

            // choose location off screen
            self.target = Vec2::NAN;
            self.vel = Vec2::new(
                gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)
            ) * self.speed;
            self.flyaway = true;
        }
        // otherwise, assume velocity needs to be calculated
        else
        {
            // throttle speed based on dist from target
            let curr_speed = (
                self.speed
                * (self.pos.distance(self.target)
                * GRAB_RAD)
            ).clamp(0.0, self.speed);

            self.vel = -(self.pos - self.target).normalize() * curr_speed;
        }

        self.fly(dt);
    }

    pub fn render(&mut self)
    {
        let x = self.pos.x - (self.rect.w / 2.0);
        let y = self.pos.y - (self.rect.h / 2.0);
        draw_texture_ex(
            self.texture,
            x,
            y,
            color::WHITE,
            DrawTextureParams
            {
                source: Some(self.anim_dat.frame().source_rect),
                flip_x: self.vel.x < 0.0,
                ..Default::default()
            }
        );

        self.anim_dat.update();
    }
}

