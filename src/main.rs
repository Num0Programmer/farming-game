use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub mod characters;
use characters::*;

pub mod plants;
use plants::*;

pub mod tools;
use tools::*;

pub mod tilemap;
use tilemap::*;

const CHARACTER_PATH: &str = "assets/characters/";
const PLANT_PATH: &str = "assets/plants/";
const TILEMAP_PATH: &str = "assets/tilemap/";
const GROUND: Color = Color::new(0.212, 0.337, 0.173, 1.0);

#[macroquad::main("Farming Game")]
async fn main()
{
    // init game management
    let mut score = 0;

    // init crop cells and grid
    let dry_t = load_texture(
        &(TILEMAP_PATH.to_owned() + "dry_soil.png")
    ).await.unwrap();
    let wet_t = load_texture(
        &(TILEMAP_PATH.to_owned() + "wet_soil.png")
    ).await.unwrap();
    let seedling_t = load_texture(
        &(PLANT_PATH.to_owned() + "seedling.png")
    ).await.unwrap();
    let mut crop_grid = CropGrid::new(
        screen_width() / 4., screen_height() / 2.,
        400., 500.
    );

    // init player and tools
    let mut player = Player::new(
        120.0,
        load_texture(
            &(CHARACTER_PATH.to_owned() + "place_holder.png")
        ).await.unwrap()
    );
    let water_can = WaterCan::new();

    // init crows
    let mut crow_1 = Crow::new(
        160.0,
        Vec2::new(
            screen_width() + SCREEN_BORDER_EXT,
            gen_range(-SCREEN_BORDER_EXT, screen_height() + SCREEN_BORDER_EXT)
        ),
        load_texture(&(CHARACTER_PATH.to_owned() + "crow.png")).await.unwrap()
    );
    let mut crow_2 = Crow::new(
        160.0,
        Vec2::new(
            screen_width() + SCREEN_BORDER_EXT,
            gen_range(-SCREEN_BORDER_EXT, screen_height() + SCREEN_BORDER_EXT)
        ),
        load_texture(&(CHARACTER_PATH.to_owned() + "crow.png")).await.unwrap()
    );

    // init plants
    let potato = PlantType::new(
        "Potato", 0.0, 10.0, 6.0,
        Texture2D::empty(),
        load_texture(&(PLANT_PATH.to_owned() + "potato.png")).await.unwrap()
    );
    let carrot = PlantType::new(
        "Carrot", 0.0, 10.0, 2.0,
        Texture2D::empty(),
        load_texture(
            &(PLANT_PATH.to_owned() + "carrot.png")
        ).await.unwrap()
    );
    let beet = PlantType::new(
        "Beet", 0.0, 15.0, 6.0,
        Texture2D::empty(),
        load_texture(&(PLANT_PATH.to_owned() + "beet.png")).await.unwrap()
    );
    let tomato = PlantType::new(
        "Tomato", 6.0, 10.0, 10.0,
        load_texture(
            &(PLANT_PATH.to_owned() + "tomato_sprout.png")
        ).await.unwrap(),
        load_texture(&(PLANT_PATH.to_owned() + "tomato.png")).await.unwrap()
    );

    let mut selected_plant = &potato;
    loop // game loop
    {
        if is_key_pressed(KeyCode::Escape)
        {
            break;
        }

        clear_background(GROUND);

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
        // otherwise, check for tomato select
        else if is_key_pressed(KeyCode::Key4)
        {
            selected_plant = &tomato;
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
        else if is_key_pressed(KeyCode::H)
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
        crow_1.update(get_frame_time(), &mut crop_grid);
        crow_2.update(get_frame_time(), &mut crop_grid);

        // draw entities to screen
        crop_grid.render(&seedling_t, &dry_t, &wet_t);
        player.render();
        crow_1.render();
        crow_2.render();

        next_frame().await
    }
}

