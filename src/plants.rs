use macroquad::color::WHITE;
use macroquad::prelude::Rect;
use macroquad::prelude::Vec2;
use macroquad::texture::*;
use macroquad::window::*;

// expected size of sprite -- useful when crop grid becomes a tilemap
const TILEMAP_SPRITE_DIM: f32 = 32.0;

const CROP_ROWS: usize = 4;
const CROPS_PER_ROW: usize = 5;

/// structure which holds information about the space in the crop grid
struct CropGridCell<'a>
{
    pos: Vec2,
    rect: Rect,
    water_level: f32,
    plant: Option<Plant<'a>>,
}

impl<'a> CropGridCell<'a>
{
    fn new(pos: Vec2) -> Self
    {
        let rect = Rect::new(pos.x, pos.y, TILEMAP_SPRITE_DIM, TILEMAP_SPRITE_DIM);
        Self
        {
            pos,
            rect,
            plant: None,
            water_level: 0.,
        }
    }

    fn update(&mut self, dt: f32)
    {
        //if there is no plant, do nothing
        if let Some(plant) = &mut self.plant
        {
            plant.update(dt, &mut self.water_level)
        }
    }

    fn plant(&mut self, plant: &'a PlantType)
    {
        if self.plant.is_none()
        {
            self.plant = Some(Plant::new(plant));
        }
    }

    fn harvest(&mut self, score: &mut i32)
    {
        if let Some(plant) = &mut self.plant
        {
            plant.harvest(score);

            if plant.plant_type.sprout_time == 0.
            {
                self.plant = None;
            }
        }
    }

    fn pull(&mut self, score: &mut i32)
    {
        // only pull if there is a plant that is fully grown
        if matches!(&self.plant, Some(plant) if plant.grow_counter <= 0.)
        {
            *score += 10;
        }

        self.plant = None;
    }

    fn render(
        &self,
        seeded_t: &Texture2D,
        dry_t: &Texture2D,
        wet_t: &Texture2D,
    ) {
        // render the ground, wet or dry
        let ground = if self.water_level > 0. { *wet_t } else { *dry_t };
        draw_texture(ground, self.pos.x, self.pos.y, WHITE);
        // only render the plant, if there is one
        if let Some(plant) = &self.plant {
            plant.render(seeded_t, self.pos.x, self.pos.y);
        }
    }

    fn water(&mut self, portion: f32)
    {
        if self.water_level <= 0.0
        {
            self.water_level = portion;
        }
    }
}

/// structure which represents places on the ground crops can be planted
pub struct CropGrid<'a>
{
    pos: Vec2,
    screen_partition: Vec2,
    crops: Vec<CropGridCell<'a>>
}

impl<'a> CropGrid<'a>
{
    pub fn new(x: f32, y: f32) -> Self
    {
        let area = CROP_ROWS * CROPS_PER_ROW;
        let pos = Vec2::new(x, y);
        let screen_partition = Vec2::new(
            screen_width() / CROPS_PER_ROW as f32,
            screen_height() / CROP_ROWS as f32
        );

        let x_init = (pos.x / CROPS_PER_ROW as f32)
            - (TILEMAP_SPRITE_DIM / 2.0);
        let y_init = (pos.y / CROP_ROWS as f32)
            - (TILEMAP_SPRITE_DIM / 2.0);
        let mut crops = Vec::with_capacity(area);
        // initialize crops
        for row in 0..CROP_ROWS
        {
            let y = y_init + (row as f32 * screen_partition.y);

            for col in 0..CROPS_PER_ROW
            {
                let x = x_init + (col as f32 * screen_partition.x);
                let pos = Vec2::new(x, y);
                crops.push(CropGridCell::new(pos));
            }
        }

        Self { pos, screen_partition, crops }
    }

    fn check_for_intersect<'b>(
        &'b mut self,
        query: Rect
    ) -> Option<&'b mut CropGridCell<'a>>
    {
        self.crops.iter_mut().find(|crop| crop.rect.intersect(query).is_some())
    }

    pub fn harvest_from_cell(&mut self, query: Rect, score: &mut i32)
    {
        if let Some(crop) = self.check_for_intersect(query)
        {
            crop.harvest(score);
        }
    }

    pub fn plant_to_cell(&mut self, plant: &'a PlantType, query: Rect)
    {
        if let Some(crop) = self.check_for_intersect(query)
        {
            crop.plant(plant);
        }
    }

    pub fn pull_from_cell(&mut self, query: Rect, score: &mut i32)
    {
        if let Some(crop) = self.check_for_intersect(query)
        {
            crop.pull(score);
        }
    }

    pub fn water_cell(&mut self, query: Rect, portion: f32)
    {
        if let Some(crop) = self.check_for_intersect(query)
        {
            crop.water(portion);
        }
    }

    pub fn update(&mut self, dt: f32)
    {
        for crop in &mut self.crops
        {
            crop.update(dt);
        }
    }

    pub fn render(
        &self,
        seedling_t: &Texture2D,
        dry_t: &Texture2D,
        watered_t: &Texture2D,
    ) {
        for crop in &self.crops
        {
            crop.render(seedling_t, dry_t, watered_t);
        }
    }
}

/// structure which represents a plant type
pub struct PlantType
{
    name: &'static str,
    sprout_time: f32,
    grow_time: f32,
    water_usage: f32,
    sprout_t: Texture2D, // for plants which sprout, then produce (i.e. tomatoes)
    plant_t: Texture2D
}

impl PlantType
{
    pub fn new(
        name: &'static str,
        sprout_time: f32,
        grow_time: f32,
        water_usage: f32,
        sprout_t: Texture2D,
        plant_t: Texture2D
    ) -> Self
    {
        Self
        {
            name,
            sprout_time,
            grow_time,
            water_usage,
            sprout_t,
            plant_t
        }
    }
}

/// structure which represents a plant instance
pub struct Plant<'a>
{
    plant_type: &'a PlantType,
    grow_counter: f32
}

impl<'a> Plant<'a>
{
    fn new(plant_type: &'a PlantType) -> Self
    {
        Self
        {
            plant_type,
            grow_counter: plant_type.grow_time + plant_type.sprout_time
        }
    }

    fn update(&mut self, dt: f32, water_level: &mut f32)
    {
        if self.grow_counter > 0. && *water_level > 0.0
        {
            *water_level -= self.plant_type.water_usage * dt;
            self.grow_counter -= dt;
        }
    }

    fn harvest(&mut self, score: &mut i32)
    {
        if self.grow_counter <= 0.
        {
            *score += 10;
            self.grow_counter = self.plant_type.sprout_time;
        }
    }

    fn render(&self, seeded_t: &Texture2D, x: f32, y: f32)
    {
        let offset = 10.0
            + (32.0 * ((self.plant_type.plant_t.height() / TILEMAP_SPRITE_DIM) - 1.0));
        if self.grow_counter <= 0.
        {
            // plant is fully grown
            draw_texture(self.plant_type.plant_t, x, y - offset, WHITE);
        }
        else if self.grow_counter <= self.plant_type.sprout_time
        {
            // plant has sprouted
            draw_texture(self.plant_type.sprout_t, x, y - offset, WHITE);
        }
        else
        {
            // plant is on it's initial stage
            draw_texture(*seeded_t, x, y - 10.0, WHITE);
        }
    }
}
