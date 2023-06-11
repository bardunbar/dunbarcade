use atlas::TextureAtlas;
use macroquad::{texture::{load_texture, Texture2D, draw_texture}, prelude::{WHITE, is_key_down}, input::KeyCode};


const JOYSTICK_FRAMES: [&str;9] = [
    "ControlUpLeft.png",    // 0
    "ControlUp.png",        // 1
    "ControlUpRight.png",   // 2
    "ControlLeft.png",      // 3
    "ControlDefault.png",   // 4
    "ControlRight.png",     // 5
    "ControlDownLeft.png",  // 6
    "ControlDown.png",      // 7
    "ControlDownRight.png", // 8
];

pub struct JoystickState(usize);

pub struct Cabinet {

    cabinet_texture: Texture2D,
    joystick_atlas: TextureAtlas,
    joystick_state: JoystickState,
}


impl Cabinet {
    pub async fn new() -> Self {
        let cabinet_texture = load_texture("assets/cabinet/arcade_cabinet.png").await.unwrap();

        let joystick_atlas = TextureAtlas::from_data("assets/atlas/joystick.json", Some("assets/atlas/joystick.png")).await.unwrap();

        Cabinet {
            cabinet_texture,
            joystick_atlas,
            joystick_state: JoystickState(4),
        }
    }

    pub fn update(&mut self) {

        let left_pressed = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let right_pressed = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let up_pressed = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let down_pressed = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);


        let joystick_x = if left_pressed && right_pressed {
            1
        } else if left_pressed {
            0
        } else if right_pressed {
            2
        } else {
            1
        };

        let joystick_y = if up_pressed && down_pressed {
            1
        } else if up_pressed {
            0
        } else if down_pressed {
            2
        } else {
            1
        };

        self.joystick_state = JoystickState(3 * joystick_y + joystick_x);
    }

    pub fn draw(&self) {
        draw_texture(self.cabinet_texture, 0.0, 0.0, WHITE);
        self.joystick_atlas.draw_texture(JOYSTICK_FRAMES[self.joystick_state.0], 16., 536., 0., WHITE);
    }
}