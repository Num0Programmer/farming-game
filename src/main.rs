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
    // init game management
    let mut score = 0;
    let mut crop_grid = CropGrid::new(
        screen_width() / 2.0, screen_height() / 2.0,
        load_texture("assets/dirt.png").await.unwrap(),
        load_texture("assets/watered_dirt.png").await.unwrap(),
        load_texture("assets/seedling.png").await.unwrap()
    );

    // init player and tools
    let mut player = Player::new(
        120.0, load_texture("assets/place_holder.png").await.unwrap()
    );
    let water_can = WaterCan::new();

    // init plants
    let potato = Plant::new(
        "Potato".to_string(), 12.0, 4.0,
        Texture2D::empty(),
        load_texture("assets/potato.png").await.unwrap()
    );
    let carrot = Plant::new(
        "Carrot".to_string(), 10.0, 7.0,
        Texture2D::empty(),
        load_texture("assets/carrot.png").await.unwrap()
    );
    let beet = Plant::new(
        "Beet".to_string(), 12.0, 2.0,
        Texture2D::empty(),
        load_texture("assets/beet.png").await.unwrap()
    );

    let mut selected_plant = &potato;
    loop // game loop
    {
        if is_key_pressed(KeyCode::Escape)
        {
            break;
        }

        clear_background(BLUE);

        // check for potato select
        if is_key_pressed(KeyCode::Key1)
        {
            selected_plant = &potato;
        }
        // otherwise, check for carrot select
        else if is_key_pressed(KeyCode::Key2)
        {
            selected_plant = &carrot;
        }
        // otherwise, check for beet select
        else if is_key_pressed(KeyCode::Key3)
        {
            selected_plant = &beet;
        }

        // check for plant button
        if is_key_pressed(KeyCode::J)
        {
            crop_grid.plant_to_cell(selected_plant, player.get_rect());
        }
        // otherwise, check for pull button
        else if is_key_pressed(KeyCode::K)
        {
            crop_grid.pull_from_cell(player.get_rect(), &mut score);
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

