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
struct CropGridCell
{
    has_plant: bool,
    pos: Vec2,
    rect: Rect,
    water_level: f32,
    seeded_t: Texture2D,
    plant: Plant
}

impl CropGridCell
{
    fn new(pos: Vec2, seedling_t: Texture2D, plant: Plant) -> Self
    {
        let rect = Rect::new(pos.x, pos.y, TILEMAP_SPRITE_DIM, TILEMAP_SPRITE_DIM);
        Self
        {
            has_plant: false,
            pos,
            rect,
            water_level: 0.0,
            seeded_t: seedling_t,
            plant
        }
    }

    fn update(&mut self, dt: f32)
    {
        if self.has_plant && !self.plant.grown && self.water_level > 0.0
        {
            self.water_level -= self.plant.water_usage * dt;
            self.plant.update(dt);
        }
    }

    fn plant(&mut self, plant: &Plant)
    {
        if !self.has_plant
        {
            self.plant.set_plant(&plant);
            self.has_plant = true;
        }
    }

    fn harvest(&mut self, score: &mut i32)
    {
        if self.has_plant && self.plant.sprouted && self.plant.grown
        {
            self.plant.current_grow_time = self.plant.grow_time;
            self.plant.grown = false;

            *score += 10;
        }
    }

    fn pull(&mut self, score: &mut i32)
    {
        if self.has_plant && self.plant.grown
        {
            self.plant.set_plant(&Plant::default());
            self.has_plant = false;

            *score += 10;
        }
    }

    fn render(&self)
    {
        let offset = 10.0
            + (32.0 * ((self.plant.plant_t.height() / TILEMAP_SPRITE_DIM) - 1.0));
        if self.has_plant && self.plant.grown
        {
            draw_texture(
                self.plant.plant_t,
                self.pos.x,
                self.pos.y - offset,
                WHITE
            );
        }
        // otherwise, check plant has sprouted
        else if self.has_plant && self.plant.sprouted
        {
            draw_texture(
                self.plant.sprout_t,
                self.pos.x,
                self.pos.y - offset,
                WHITE
            );
        }
        // otherwise, assume communication there is something in the cell
        else if self.has_plant
        {
            draw_texture(
                self.seeded_t,
                self.pos.x,
                self.pos.y - 10.0,
                WHITE
            );
        }
    }

    fn has_water(&self) -> bool
    {
        self.water_level > 0.0
    }

    fn water(&mut self, portion: f32)
    {
        if !self.has_water()
        {
            self.water_level = portion;
        }
    }
}

/// structure which represents places on the ground crops can be planted
pub struct CropGrid
{
    pos: Vec2,
    screen_partition: Vec2,
    dry_t: Texture2D,
    watered_t: Texture2D,
    crops: Vec<CropGridCell>
}

impl CropGrid
{
    pub fn new(
        x: f32, y: f32,
        dry_t: Texture2D,
        watered_t: Texture2D,
        seedling_t: Texture2D
    ) -> Self
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
                - (TILEMAP_SPRITE_DIM / 2.0);
            let mut y = (pos.y / CROP_ROWS as f32)
                - (TILEMAP_SPRITE_DIM / 2.0);

            for _row in 0..CROP_ROWS
            {
                let mut x = x_init;
                for _col in 0..CROPS_PER_ROW
                {
                    let pos = Vec2::new(x, y);
                    crops.push(CropGridCell::new(
                        pos, seedling_t, Plant::default()
                    ));
                    x += screen_partition.x;
                }
                y += screen_partition.y;
            }
        }

        Self { pos, screen_partition, dry_t, watered_t, crops }
    }

    pub fn check_for_intersect(
        &mut self,
        index: &mut i32,
        query: Rect
    ) -> Rect
    {
        for i in 0..self.crops.len()
        {
            let intersect = self.crops[i].rect.intersect(query);

            match intersect
            {
                Some(intersect) => {
                    *index = i as i32;
                    return intersect
                },
                None => continue
            }
        }

        *index = -1;
        Rect::default()
    }

    pub fn harvest_from_cell(&mut self, query: Rect, score: &mut i32)
    {
        let mut crop_index: i32 = -1;
        let intersect = self.check_for_intersect(&mut crop_index, query);

        if crop_index > -1
        {
            let crop = &mut self.crops[crop_index as usize];
            crop.harvest(score)
        }
    }

    pub fn plant_to_cell(&mut self, plant: &Plant, query: Rect)
    {
        let mut crop_index: i32 = -1;
        let intersect = self.check_for_intersect(&mut crop_index, query);

        if crop_index > -1
        {
            let crop = &mut self.crops[crop_index as usize];
            crop.plant(plant)
        }
    }

    pub fn pull_from_cell(&mut self, query: Rect, score: &mut i32)
    {
        let mut crop_index: i32 = -1;
        let intersect = self.check_for_intersect(&mut crop_index, query);

        if crop_index > -1
        {
            let crop = &mut self.crops[crop_index as usize];
            crop.pull(score);
        }
    }

    pub fn water_cell(&mut self, query: Rect, portion: f32)
    {
        let mut crop_index: i32 = -1;
        let intersect = self.check_for_intersect(&mut crop_index, query);

        if crop_index > -1
        {
            let crop = &mut self.crops[crop_index as usize];
            crop.water(portion);
        }
    }

    pub fn update(&mut self, dt: f32)
    {
        for i in 0..self.crops.len()
        {
            self.crops[i].update(dt);
        }
    }

    pub fn render(&self)
    {
        for crop in &self.crops
        {
            if !crop.has_water()
            {
                draw_texture(
                    self.dry_t,
                    crop.pos.x,
                    crop.pos.y,
                    WHITE
                );
            }
            else
            {
                draw_texture(
                    self.watered_t,
                    crop.pos.x,
                    crop.pos.y,
                    WHITE
                );
            }

            crop.render();
        }
    }
}

/// structure which represents a plant
pub struct Plant
{
    grown: bool,
    sprouted: bool,
    name: &'static str,
    sprout_time: f32,
    grow_time: f32,
    current_grow_time: f32,
    water_usage: f32,
    sprout_t: Texture2D, // for plants which sprout, then produce (i.e. tomatoes)
    plant_t: Texture2D
}

impl Default for Plant {
    fn default() -> Self
    {
        Self
        {
            grown: false,
            sprouted: false,
            name: "",
            sprout_time: 0.0,
            grow_time: 0.0,
            current_grow_time: 0.0,
            water_usage: 0.0,
            sprout_t: Texture2D::empty(),
            plant_t: Texture2D::empty()
        }
    }

}

impl Plant
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
            grown: false,
            sprouted: false,
            name,
            sprout_time,
            grow_time,
            current_grow_time: grow_time,
            water_usage,
            sprout_t,
            plant_t
        }
    }

    fn update(&mut self, dt: f32)
    {
        self.sprout_time -= dt;
        if self.sprout_time <= 0.0
        {
            self.current_grow_time -= dt;
            self.sprouted = self.sprout_t.ne(&Texture2D::empty());
        }

        if self.current_grow_time <= 0.0
        {
            self.current_grow_time = 0.0;
            self.grown = true;
        }
    }

    fn set_plant(&mut self, plant: &Plant)
    {
        self.sprouted = plant.sprouted;
        self.grown = plant.grown;
        self.name = plant.name.clone();
        self.sprout_time = plant.sprout_time;
        self.grow_time = plant.grow_time;
        self.current_grow_time = plant.current_grow_time;
        self.water_usage = plant.water_usage;
        self.sprout_t = plant.sprout_t;
        self.plant_t = plant.plant_t;
    }
}

