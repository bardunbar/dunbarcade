use std::collections::HashMap;

use atlas::{TextureAtlas, AtlasTextureParams};
use macroquad::{time::get_frame_time, prelude::{Vec2, WHITE, is_key_down, KeyCode}};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum PlayerAnimation {
    MoveDownRight,
    MoveDownLeft,
    MoveUpRight,
    MoveUpLeft,
    StoppedLeft,
    StoppedRight,
}

pub struct AnimDef {
    frames: Vec<String>,
    flip_x: bool,
}

pub struct Player {
    atlas: TextureAtlas,
    animations: HashMap<PlayerAnimation, AnimDef>,
    animation: PlayerAnimation,
    frame: usize,
    anim_speed: f32,
    frame_counter: f32,
    move_speed: f32, // Pixels per second

    pub position: Vec2,
}

impl Player {
    pub async fn new() -> Self {
        let atlas = TextureAtlas::from_data("assets/atlas/player.json", Some("assets/atlas/player.png")).await.unwrap();

        let mut animations = HashMap::new();

        animations.insert(PlayerAnimation::MoveDownRight, AnimDef {
            frames: vec![
                "player_down_0.png".to_string(),
                "player_down_1.png".to_string(),
                "player_down_2.png".to_string(),
                "player_down_3.png".to_string(),
            ],
            flip_x: false
        });

        animations.insert(PlayerAnimation::MoveDownLeft, AnimDef {
            frames: vec![
                "player_down_0.png".to_string(),
                "player_down_1.png".to_string(),
                "player_down_2.png".to_string(),
                "player_down_3.png".to_string(),
            ],
            flip_x: true
        });

        animations.insert(PlayerAnimation::MoveUpRight, AnimDef {
            frames: vec![
                "player_up_0.png".to_string(),
                "player_up_1.png".to_string(),
                "player_up_2.png".to_string(),
                "player_up_3.png".to_string(),
            ],
            flip_x: false
        });

        animations.insert(PlayerAnimation::MoveUpLeft, AnimDef {
            frames: vec![
                "player_up_0.png".to_string(),
                "player_up_1.png".to_string(),
                "player_up_2.png".to_string(),
                "player_up_3.png".to_string(),
            ],
            flip_x: true
        });

        animations.insert(PlayerAnimation::StoppedRight, AnimDef {
            frames: vec![
                "player_down_0.png".to_string(),
            ],
            flip_x: false
        });

        animations.insert(PlayerAnimation::StoppedLeft, AnimDef {
            frames: vec![
                "player_down_0.png".to_string(),
            ],
            flip_x: true
        });

        Player {
            atlas,
            animations,
            animation: PlayerAnimation::MoveDownLeft,
            frame: 0,
            anim_speed: 1.0 / 12.0,
            frame_counter: 0.0,
            move_speed: 96.0, // Pixels per second
            position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn update(&mut self) {

        let frame_time = get_frame_time();

        let mut move_dir = Vec2::ZERO;

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_dir.x -= 1.0;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_dir.x += 1.0;
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_dir.y -= 1.0;
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_dir.y += 1.0;
        }

        move_dir = move_dir.normalize_or_zero();

        move_dir *= self.move_speed * frame_time;


        let target_anim_state = match (move_dir.x, move_dir.y) {
            (x, y) if x > 0.0 && y >= 0.0 => {PlayerAnimation::MoveDownRight},
            (x, y) if x == 0.0 && y > 0.0 => {
                if self.animation == PlayerAnimation::MoveDownLeft || self.animation == PlayerAnimation::StoppedLeft {
                    PlayerAnimation::MoveDownLeft
                } else {
                    PlayerAnimation::MoveDownRight
                }
            },
            (x, y) if x < 0.0 && y >= 0.0 => {PlayerAnimation::MoveDownLeft},
            (x, y) if x >= 0.0 && y < 0.0 => {PlayerAnimation::MoveUpRight},
            (x, y) if x <= 0.0 && y < 0.0 => {PlayerAnimation::MoveUpLeft},
            _ => {
                match self.animation {
                    PlayerAnimation::MoveDownLeft => PlayerAnimation::StoppedLeft,
                    PlayerAnimation::MoveUpLeft => PlayerAnimation::StoppedLeft,
                    PlayerAnimation::StoppedLeft => PlayerAnimation::StoppedLeft,
                    _ => PlayerAnimation::StoppedRight,
                }
            }
        };

        self.position += move_dir;
        self.position.round();

        if target_anim_state != self.animation {
            self.animation = target_anim_state;
            self.frame = 0;
        }

        // Update the animation based on the current state
        let current_animation = self.animations.get(&self.animation);
        if let Some(anim_def) = current_animation {
            self.frame_counter -= frame_time;

            if self.frame_counter <= 0.0 {
                self.frame += 1;

                if self.frame >= anim_def.frames.len() {
                    self.frame = 0;
                }

                self.frame_counter = self.anim_speed;
            }
        }

    }

    pub fn draw(&self) {

        if let Some(anim_def) = self.animations.get(&self.animation) {
            self.atlas.draw_texture_params(&anim_def.frames[self.frame], self.position.x, self.position.y, WHITE, AtlasTextureParams {
                flip_x: anim_def.flip_x,
                ..Default::default()
            })
        }

    }
}