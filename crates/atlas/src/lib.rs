use std::{collections::HashMap, path::Path};

use nanoserde::DeJson;
use macroquad::{
    texture::{
        Texture2D,
        load_texture,
        DrawTextureParams,
        draw_texture_ex
    },
    prelude::{
        load_string,
        Color,
        Rect, Vec2
    },
};

#[derive(DeJson, Clone, Copy)]
struct FrameRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[allow(dead_code)]
#[derive(DeJson)]
struct AtlasSize {
    w: f32,
    h: f32,
}

#[allow(dead_code)]
#[derive(DeJson)]
struct FrameData {
    filename: Option<String>,
    frame: FrameRect,
    rotated: bool,
    trimmed: bool,
    #[nserde(rename = "spriteSourceSize")]
    sprite_source_size: FrameRect,
    #[nserde(rename = "sourceSize")]
    source_size: AtlasSize,
}

#[allow(dead_code)]
#[derive(DeJson)]
struct MetaData {
    app: Option<String>,
    version: Option<String>,
    image: Option<String>,
    format: Option<String>,
    size: AtlasSize,
    scale: Option<String>,
    #[nserde(rename = "smartupdate")]
    smart_update: Option<String>,
}

#[derive(DeJson)]
struct AtlasData {
    frames: Vec<FrameData>,
    meta: MetaData,
}

#[allow(dead_code)]
pub struct TextureAtlas {
    data: AtlasData,
    texture: Texture2D,
    frames: HashMap<String, FrameRect>
}

#[derive(Debug, Clone)]
pub struct AtlasTextureParams {
    /// Rotation in radians
    pub rotation: f32,

    /// Mirror on the X axis
    pub flip_x: bool,

    /// Mirror on the Y axis
    pub flip_y: bool,

    /// Rotate around this point.
    /// When `None`, rotate around the texture's center.
    /// When `Some`, the coordinates are in screen-space.
    /// E.g. pivot (0,0) rotates around the top left corner of the screen, not of the
    /// texture.
    pub pivot: Option<Vec2>,
}

impl Default for AtlasTextureParams {
    fn default() -> AtlasTextureParams {
        AtlasTextureParams {
            rotation: 0.,
            pivot: None,
            flip_x: false,
            flip_y: false,
        }
    }
}

impl TextureAtlas {
    pub async fn from_data(data_path: &str, texture_path: Option<&str>) -> Result<Self, String> {
        if let Ok(contents) = load_string(data_path).await {
            if let Ok(mut atlas) = AtlasData::deserialize_json(&contents) {
                let path = Path::new(&data_path);
                let parent_path = path.parent().unwrap();
                let t_path_buf = parent_path.join(&atlas.meta.image.as_ref().unwrap());

                if let Ok(new_path) = t_path_buf.into_os_string().into_string(){
                    atlas.meta.image = Some(new_path);
                }

                let texture_path = match texture_path {
                    Some(path) => path,
                    None => {
                        &atlas.meta.image.as_ref().unwrap()
                    },
                };
                if let Ok(texture) = load_texture(texture_path).await {
                    let mut frames = HashMap::new();
                    for frame in atlas.frames.iter() {
                        frames.insert(frame.filename.as_ref().unwrap().to_owned(), frame.frame.clone());
                    }

                    // Set the filter mode to be nearest for pixel perfection!
                    texture.set_filter(macroquad::texture::FilterMode::Nearest);

                    Ok(TextureAtlas {
                        data: atlas,
                        texture,
                        frames
                    })
                }
                else {
                    Err(format!("Unable to load texture image at path {}", texture_path))
                }
            }
            else {
                Err(format!("Unable to deserialize AtlasData at path {}", data_path))
            }
        }
        else {
            Err(format!("Unable to load AtlasData at path {}", data_path))
        }
    }

    pub fn draw_texture(&self, texture: &str, x: f32, y: f32, rotation: f32, color: Color) {
        if let Some(frame_data) = self.frames.get(texture) {
            draw_texture_ex(self.texture, x, y, color, DrawTextureParams {
                source: Some(Rect {x: frame_data.x, y: frame_data.y, w: frame_data.w, h: frame_data.h } ),
                rotation,
                ..Default::default()
            })
        }
    }

    pub fn draw_texture_params(&self, texture: &str, x: f32, y: f32, color: Color, params: AtlasTextureParams) {
        if let Some(frame_data) = self.frames.get(texture) {
            draw_texture_ex(self.texture, x, y, color, DrawTextureParams {
                source: Some(Rect {x: frame_data.x, y: frame_data.y, w: frame_data.w, h: frame_data.h } ),
                rotation: params.rotation,
                flip_x: params.flip_x,
                flip_y: params.flip_y,
                pivot: params.pivot,
                ..Default::default()
            })
        }
    }
}