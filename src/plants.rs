use macroquad::color::WHITE;
use macroquad::prelude::Rect;
use macroquad::prelude::Vec2;
use macroquad::texture::*;
use macroquad::window::*;

const SPRITE_DIM: f32 = 32.0;

const CROP_ROWS: usize = 4;
const CROPS_PER_ROW: usize = 5;

/// structure which holds information about the space in the crop grid
struct CropGridCell
{
    has_water: bool,
    pos: Vec2,
    rect: Rect,
    plant: Plant
}

impl CropGridCell
{
    fn new(pos: Vec2, plant: Plant) -> Self
    {
        let rect = Rect::new(pos.x, pos.y, SPRITE_DIM, SPRITE_DIM);
        Self { has_water: false, pos, rect, plant }
    }
}

/// structure which represents places on the ground crops can be planted
// TODO: might need to do something about the planter not know which crop the
// player wants to remove
pub struct CropGrid
{
    pos: Vec2,
    screen_partition: Vec2,
    texture: Texture2D,
    crops: Vec<CropGridCell>
}

impl CropGrid
{
    pub fn new(x: f32, y: f32, texture: Texture2D) -> Self
    {
        let area = CROP_ROWS * CROPS_PER_ROW;
        let pos = Vec2::new(x, y);
        let screen_partition = Vec2::new(
            screen_width() / CROPS_PER_ROW as f32,
            screen_height() / CROP_ROWS as f32
        );

        let mut crops = Vec::with_capacity(area);
        // initialize crops
        {
            let x_init = (pos.x / CROPS_PER_ROW as f32)
                - (texture.width() / 2.0) - 2.0;
            let mut y = (pos.y / CROP_ROWS as f32)
                - (texture.height() / 2.0);

            for _row in 0..CROP_ROWS
            {
                let mut x = x_init;
                for _col in 0..CROPS_PER_ROW
                {
                    let pos = Vec2::new(x, y);
                    crops.push(
                        CropGridCell::new(
                            pos, Plant::new(
                                "no plant".to_string(), 0.0,
                                Texture2D::empty(), Texture2D::empty()
                            )
                        )
                    );
                    x += screen_partition.x;
                }
                y += screen_partition.y;
            }
        }

        Self { pos, screen_partition, texture, crops }
    }

    pub fn render(&self)
    {
        for crop in &self.crops
        {
            if !crop.has_water
            {
                draw_texture(
                    self.texture,
                    crop.pos.x,
                    crop.pos.y,
                    WHITE
                );
            }
            else
            {
                panic!("No textured for a watered crop!\n");
            }
        }
    }
}

/// structure which represents a plant
pub struct Plant
{
    grown: bool,
    name: String,
    grow_t: f32,
    seed_texture: Texture2D,
    plant_texture: Texture2D
}

impl Plant
{
    pub fn new(
        name: String, grow_t: f32,
        seed_texture: Texture2D,
        plant_texture: Texture2D
    ) -> Self
    {
        Self { grown: false, name, grow_t, seed_texture, plant_texture }
    }
}

