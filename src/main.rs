use macroquad::prelude::*;

pub mod characters;
use characters::Player;

pub mod plants;
use plants::*;

pub mod tools;
use tools::*;

#[macroquad::main("Farming Game")]
async fn main()
{
    let mut score = 0;
    let dirt_t = load_texture("assets/dirt.png").await.unwrap();
    let watered_t = load_texture("assets/watered_dirt.png").await.unwrap();
    let player_texture = load_texture("assets/place_holder.png").await.unwrap();
    let potato_texture = load_texture("assets/potato.png").await.unwrap();

    let mut player = Player::new(120.0, player_texture);
    let mut crop_grid = CropGrid::new(screen_width() / 2.0, screen_height() / 2.0, dirt_t, watered_t);
    let water_can = WaterCan::new();
    let potato = Plant::new("Potato".to_string(), 0.5, 0.1, Texture2D::empty(), potato_texture);

    loop // game loop
    {
        if is_key_pressed(KeyCode::Escape)
        {
            break;
        }

        clear_background(BLUE);

        // check for plant button
        if is_key_pressed(KeyCode::J)
        {
            crop_grid.plant_to_cell(&potato, player.get_rect());
        }
        // otherwise, check for harvest button
        else if is_key_pressed(KeyCode::K)
        {
            crop_grid.harvest_from_cell(player.get_rect(), &mut score);
            println!("Score: {}", score);
        }
        // otherwise, check for water button
        else if is_key_pressed(KeyCode::L)
        {
            crop_grid.water_cell(player.get_rect(), water_can.get_portion());
        }

        // update entities
        crop_grid.update(get_frame_time());
        player.update(get_frame_time());

        // draw entities to screen
        crop_grid.render();
        player.render();

        next_frame().await
    }
}

