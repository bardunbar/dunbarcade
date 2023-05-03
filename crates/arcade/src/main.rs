use atlas::TextureAtlas;
use camera_layer::CameraLayer;
use macroquad::{window::{Conf, next_frame, clear_background}, prelude::{is_quit_requested, WHITE, set_default_camera, set_camera, is_key_down, KeyCode, Vec2, BLACK}};
use utilities::config::ConfigSettings;


const WIDTH: i32 = 640;
const HEIGHT: i32 = 360;

#[cfg(debug_assertions)]
const DEBUG_SCREEN_SCALE: i32 = 2;
#[cfg(not(debug_assertions))]
const DEBUG_SCREEN_SCALE: i32 = 1;

const WINDOW_WIDTH: i32 = WIDTH * DEBUG_SCREEN_SCALE;
const WINDOW_HEIGHT: i32 = HEIGHT * DEBUG_SCREEN_SCALE;



fn window_conf() -> Conf {
    Conf {
        window_title: "DUNBARCADE".to_string(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut config = ConfigSettings::new();
    config.load_ini("config", "assets/config.ini").await;


    let atlas = TextureAtlas::from_data("assets/atlas/arcade_basic.json", Some("assets/atlas/arcade_basic.png")).await.unwrap();


    let mut camera_layer = CameraLayer::new(WIDTH as f32, HEIGHT as f32);

    loop {


        let mut camera_movement = Vec2::ZERO;
        const SPEED: f32 = 1.0;

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            camera_movement.x -= SPEED;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            camera_movement.x += SPEED;
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            camera_movement.y -= SPEED;
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            camera_movement.y += SPEED;
        }

        camera_layer.translate(camera_movement.x, camera_movement.y);

        set_camera(&camera_layer.camera);

        clear_background(BLACK);

        atlas.draw_texture("arcade_basic_carpet.png", 0., 0., WHITE);
        atlas.draw_texture("arcade_basic_carpet.png", 32., 0., WHITE);
        atlas.draw_texture("arcade_basic_carpet.png", 64., 0., WHITE);
        atlas.draw_texture("arcade_basic_carpet.png", 96., 0., WHITE);
        atlas.draw_texture("arcade_basic_carpet.png", 128., 0., WHITE);

        atlas.draw_texture("arcade_basic_carpet.png", 0., 32., WHITE);
        atlas.draw_texture("arcade_basic_carpet.png", 32., 32., WHITE);
        atlas.draw_texture("arcade_basic_carpet.png", 64., 32., WHITE);
        atlas.draw_texture("arcade_basic_carpet.png", 96., 32., WHITE);
        atlas.draw_texture("arcade_basic_carpet.png", 128., 32., WHITE);

        set_default_camera();

        camera_layer.draw();

        if is_quit_requested() || is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}