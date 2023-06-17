use macroquad::prelude::*;

pub mod character;
use characters::Player;

#[macroquad::main("Farming Game")]
async fn main()
{
    let p_texture = load_texture("assets/place_holder.png").await.unwrap();
    let mut player = Player::new(120.0, p_texture);

    loop // game loop
    {
        if is_key_pressed(KeyCode::Escape)
        {
            break;
        }

        clear_background(BLUE);

        player.update(get_frame_time());
        player.render();

        next_frame().await
    }
}

