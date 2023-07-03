use macroquad::prelude::*;

pub mod characters;
use characters::Player;

pub mod plants;
use plants::*;

#[macroquad::main("Farming Game")]
async fn main()
{
    let dirt_t = load_texture("assets/dirt.png").await.unwrap();
    let watered_t = load_texture("assets/watered_dirt.png").await.unwrap();
    let player_texture = load_texture("assets/place_holder.png").await.unwrap();
    let potato_texture = load_texture("assets/potato.png").await.unwrap();

    let mut player = Player::new(120.0, player_texture);
    let mut crop_grid = CropGrid::new(screen_width() / 2.0, screen_height() / 2.0, dirt_t, watered_t);
    let potato = Plant::new("Potato".to_string(), 0.0, 0.0, Texture2D::empty(), potato_texture);

    loop // game loop
    {
        if is_key_pressed(KeyCode::Escape)
        {
            break;
        }

        clear_background(BLUE);

        // update entities
        crop_grid.update(get_frame_time());
        player.update(get_frame_time());

        // draw entities to screen
        crop_grid.render();
        player.render();

        next_frame().await
    }
}

