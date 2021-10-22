use macroquad::prelude::Vec2;

pub trait Blend {
    fn blend(&self, previous: &Self, alpha: f32) -> Self;
}

impl Blend for Vec2 {
    fn blend(&self, previous: &Self, alpha: f32) -> Self {
        (*self * alpha) + (*previous * (1.0 - alpha))
    }
}

impl Blend for f32 {
    fn blend(&self, previous: &Self, alpha: f32) -> Self {
        (*self * alpha) + (*previous * (1.0 - alpha))
    }
}
