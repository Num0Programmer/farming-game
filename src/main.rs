use macroquad::prelude::*;

pub mod characters;
use characters::Player;

pub mod plants;
use plants::*;

#[macroquad::main("Farming Game")]
async fn main()
{
    let dirt_texture = load_texture("assets/dirt.png").await.unwrap();
    let player_texture = load_texture("assets/place_holder.png").await.unwrap();

    let mut player = Player::new(120.0, player_texture);
    let crop_row = CropRow::new(200.0, 300.0, dirt_texture);

    loop // game loop
    {
        if is_key_pressed(KeyCode::Escape)
        {
            break;
        }

        clear_background(BLUE);

        // update entities
        player.update(get_frame_time());

        // draw entities to screen
        crop_row.render();
        player.render();

        next_frame().await
    }
}

