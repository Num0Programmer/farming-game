use macroquad::color::WHITE;
use macroquad::prelude::Rect;
use macroquad::prelude::Vec2;
use macroquad::texture::*;

// expected size of sprite -- useful when crop grid becomes a tilemap
const SPRITE_DIM: f32 = 32.0;

const CROP_ROWS: usize = 5;
const CROPS_PER_ROW: usize = 5;
const GRID_PADDING: f32 = 22.0;

/// structure which holds information about the space in the crop grid
#[derive(Clone, Copy)]
pub struct CropGridCell<'a>
{
    pub pos: Vec2,
    pub rect: Rect,
    water_level: f32,
    pub plant: Option<Plant<'a>>,
}

impl<'a> CropGridCell<'a>
{
    fn new(pos: Vec2) -> Self
    {
        let rect = Rect::new(pos.x, pos.y, SPRITE_DIM, SPRITE_DIM);
        Self
        {
            pos,
            rect,
            plant: None,
            water_level: 0.,
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

    fn plant(&mut self, plant: &'a PlantType)
    {
        if self.plant.is_none()
        {
            self.plant = Some(Plant::new(plant));
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
        draw_texture(
            ground,
            self.pos.x - (self.rect.w / 2.),
            self.pos.y - (self.rect.h / 2.),
            WHITE
        );
        // only render the plant, if there is one
        if let Some(plant) = &self.plant {
            plant.render(
                seeded_t,
                self.pos.x - (self.rect.w / 2.),
                self.pos.y - (self.rect.h / 2.)
            );
        }
    }

    fn steal(&mut self)
    {
        self.plant = None;
    }

    fn update(&mut self, dt: f32)
    {
        //if there is no plant, do nothing
        if let Some(plant) = &mut self.plant
        {
            plant.update(dt, &mut self.water_level)
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
    w: f32,
    h: f32,
    pub crops: Vec<CropGridCell<'a>>
}

impl<'a> CropGrid<'a>
{
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self
    {
        let pos = Vec2::new(x, y);
        let area_partition = Vec2::new(
            w / CROPS_PER_ROW as f32,
            h / CROP_ROWS as f32
        );

        let x_init = pos.x - (w / 2.);
        let y_init = pos.y - (h / 2.);
        let x_padding = area_partition.x / 2.;
        let y_padding = area_partition.y / 2.;

        let mut crops = Vec::with_capacity(CROP_ROWS * CROPS_PER_ROW);
        // initialize crops
        for row in 0..CROP_ROWS
        {
            let y = y_init + (area_partition.y * row as f32) + y_padding;

            for col in 0..CROPS_PER_ROW
            {
                let x = x_init + (area_partition.x * col as f32) + x_padding;
                let pos = Vec2::new(x, y);
                crops.push(CropGridCell::new(pos));
            }
        }

        Self { pos, w, h, crops }
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

    pub fn steal_from_cell(&mut self, check: Vec2)
    {
        if let Some(cell) = self.crops.iter_mut().find(|cell| check.eq(&cell.pos))
        {
            cell.steal();
        }
    }

    pub fn water_cell(&mut self, query: Rect, portion: f32)
    {
        if let Some(crop) = self.check_for_intersect(query)
        {
            crop.water(portion);
        }
    }
}

/// structure which represents a plant type
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
        let offset = 14.0
            + (32.0 * ((self.plant_type.plant_t.height() / SPRITE_DIM) - 1.0));
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
            draw_texture(*seeded_t, x, y - 14., WHITE);
        }
    }
}

