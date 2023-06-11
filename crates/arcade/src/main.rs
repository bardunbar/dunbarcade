mod player;
mod cabinet;
mod wave_function_arcade;

use cabinet::Cabinet;
use wave_function_arcade::{TileSet, Arcade};
use atlas::TextureAtlas;
use camera_layer::CameraLayer;
use player::Player;

use macroquad::{window::{Conf, next_frame, clear_background}, prelude::{is_quit_requested, set_default_camera, set_camera, is_key_down, KeyCode, Vec2, BLACK}, audio::{load_sound, play_sound, PlaySoundParams}};

const WIDTH: i32 = 480;
const HEIGHT: i32 = 640;

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

    let cabinet_layer = CameraLayer::new(WIDTH as f32, HEIGHT as f32);
    // Offset the camera so that the target is in the center of the viewport
    let mut arcade_layer = CameraLayer::new_with_offset(WIDTH as f32, HEIGHT as f32, Vec2::new(1.0, 1.0));

    let atlas = TextureAtlas::from_data("assets/atlas/arcade_basic.json", Some("assets/atlas/arcade_basic.png")).await.unwrap();

    let tile_set = TileSet::from_basic(atlas, Vec2::new(32.0, 32.0));

    let music = load_sound("assets/audio/music/secret_of_tiki_island.ogg").await.unwrap();
    play_sound(music, PlaySoundParams {
        looped: true,
        volume: 1.0,
    });

    let mut cabinet = Cabinet::new().await;

    let arcade = Arcade::new();

    arcade_layer.translate(8.0 * 32.0, 8.0 * 32.0);

    let mut player = Player::new().await;

    player.position = Vec2::new(8.0 * 32.0, 8.0 * 32.0);

    loop {

        cabinet.update();

        player.update();

        let player_screen_pos = arcade_layer.world_to_screen(player.position);
        let safe_min = Vec2::new(128.0 / WIDTH as f32 * WINDOW_WIDTH as f32, 192.0 / HEIGHT as f32 * WINDOW_HEIGHT as f32);
        let safe_max = Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32) - safe_min;

        let player_min_delta = player_screen_pos - safe_min;
        let player_max_delta = player_screen_pos - safe_max;

        if player_min_delta.x < 0.0 || player_max_delta.x > 0.0 {
            arcade_layer.translate(player.velocity.x, 0.);
        }
        if player_min_delta.y < 0.0 || player_max_delta.y > 0.0 {
            arcade_layer.translate(0., player.velocity.y);
        }

        set_camera(&arcade_layer.camera);

        clear_background(BLACK);

        for x in -1..=1 {
            for y in -1..=1 {
                if let Some(sector) = arcade.sectors.get(x, y) {
                    sector.draw(&tile_set);
                }
            }
        }

        player.draw();

        set_camera(&cabinet_layer.camera);

        cabinet.draw();

        set_default_camera();


        arcade_layer.draw();
        cabinet_layer.draw();

        if is_quit_requested() || is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}