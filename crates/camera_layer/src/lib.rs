// use macroquad::prelude::*;

use macroquad::{
    prelude::{
        Camera2D,
        Rect,
        Vec2,
        WHITE
    },
    texture::{
        render_target,
        FilterMode,
        draw_texture_ex,
        DrawTextureParams,
        Texture2D
    },
    window::{
        screen_width,
        screen_height
    }
};

pub struct CameraLayer {
    pub camera: Camera2D,
}

impl CameraLayer {
    pub fn new(width: f32, height: f32) -> Self {
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height));
        let render_target = render_target(width as u32, height as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
        camera.render_target = Some(render_target);

        // Flip Vertically? Apparently fixes a bug
        camera.zoom.y = -camera.zoom.y;

        CameraLayer { camera }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let mut cur = self.get_pos();
        cur.x += x;
        cur.y += y;

        self.set_pos(cur);
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.camera.offset = pos;
    }

    pub fn draw(&self) {
        self.draw_ex(screen_width(), screen_height());
    }

    pub fn draw_ex(&self, target_width: f32, target_height: f32) {

        let (left_padding, top_padding, dimensions) = self.get_size_and_padding(target_width, target_height);

        draw_texture_ex(
            *self.get_texture(),
            left_padding,
            top_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dimensions),
                ..Default::default()
            }
        )
    }

    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        self.camera.screen_to_world(point)
    }

    #[inline]
    pub fn get_width(&self) -> f32 {
        self.get_texture().width()
    }

    #[inline]
    pub fn get_height(&self) -> f32 {
        self.get_texture().height()
    }

    #[inline]
    pub fn get_texture(&self) -> &Texture2D {
        &self.camera.render_target.as_ref().unwrap().texture
    }

    #[inline]
    pub fn get_pos(&self) -> Vec2 {
        self.camera.offset
    }

    pub fn get_size(&self, target_width: f32, target_height: f32) -> Vec2 {
        let scale_factor = self.get_min_scale_factor(target_width, target_height);

        Vec2::new(self.get_width() * scale_factor, self.get_height() * scale_factor)
    }

    pub fn get_size_and_padding(&self, target_width: f32, target_height: f32) -> (f32, f32, Vec2) {
        let size = self.get_size(target_width, target_height);

        let left_padding = (target_width - size.x) / 2.0;
        let top_padding = (target_height - size.y) / 2.0;

        (left_padding, top_padding, size)
    }

    pub fn get_scale_factor(&self, target_width: f32, target_height: f32) -> (f32, f32) {
        (target_width / self.get_width(), target_height / self.get_height())
    }

    pub fn get_min_scale_factor(&self, target_width: f32, target_height: f32) -> f32 {
        let (scale_width, scale_height) = self.get_scale_factor(target_width, target_height);

        f32::min(scale_width, scale_height)
    }
}