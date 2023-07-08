const DEFAULT_CAPACITY: i32 = 3;

pub struct WaterCan
{
    portion: f32,
    portions_left: i32
}

impl WaterCan
{
    pub fn new() -> Self
    {
        let portion = 10.0;
        let portions_left = DEFAULT_CAPACITY;

        Self { portion, portions_left }
    }

    pub fn get_portion(&self) -> f32
    {
        self.portion
    }
}

