use macroquad::prelude::*;

#[macroquad::main("Farming Game")]
async fn main()
{
    loop // game loop
    {
        clear_background(BLUE);

        draw_text("HELLO!", 20.0, 150.0, 250.0, WHITE);

        next_frame().await
    }
}
