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

const BUTTON_FRAMES: [&str;4] = [
    "BlueButtonUp.png",   // 0
    "BlueButtonDown.png", // 1
    "RedButtonUp.png",    // 2
    "RedButtonDown.png",  // 3
];

struct JoystickState(usize);
struct ButtonState(usize);

pub struct Cabinet {

    cabinet_texture: Texture2D,
    joystick_atlas: TextureAtlas,
    joystick_state: JoystickState,
    blue_button_state: ButtonState,
    red_button_state: ButtonState,
}


impl Cabinet {
    pub async fn new() -> Self {
        let cabinet_texture = load_texture("assets/cabinet/arcade_cabinet.png").await.unwrap();

        let joystick_atlas = TextureAtlas::from_data("assets/atlas/joystick_buttons.json", Some("assets/atlas/joystick_buttons.png")).await.unwrap();

        Cabinet {
            cabinet_texture,
            joystick_atlas,
            joystick_state: JoystickState(4),
            blue_button_state: ButtonState(0),
            red_button_state: ButtonState(2),
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


        self.blue_button_state = if is_key_down(KeyCode::Space) { ButtonState(1) } else { ButtonState(0) };
        self.red_button_state = if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) { ButtonState(3) } else { ButtonState(2) };

    }

    pub fn draw(&self) {
        draw_texture(self.cabinet_texture, 0.0, 0.0, WHITE);
        self.joystick_atlas.draw_texture(JOYSTICK_FRAMES[self.joystick_state.0], 16., 536., 0., WHITE);
        self.joystick_atlas.draw_texture(BUTTON_FRAMES[self.blue_button_state.0], 160., 573., 0., WHITE);
        self.joystick_atlas.draw_texture(BUTTON_FRAMES[self.red_button_state.0], 160., 573., 0., WHITE);
    }
}