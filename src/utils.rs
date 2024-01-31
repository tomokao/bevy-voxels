use bevy::{prelude::*, window::WindowResolution};

pub(crate) trait AsVec2 {
    fn as_vec2(&self) -> Vec2;
}

impl AsVec2 for WindowResolution {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }
}
