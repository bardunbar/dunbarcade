use macroquad::{window::{Conf, next_frame}, prelude::is_quit_requested};
use utilities::config::ConfigSettings;


fn window_conf() -> Conf {
    Conf {
        window_title: "DUNBARCADE".to_string(),
        fullscreen: false,
        window_width: 1024,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut config = ConfigSettings::new();
    config.load_ini("config", "assets/config.ini").await;


    loop {

        if is_quit_requested() {
            break;
        }

        next_frame().await;
    }
}